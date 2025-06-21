pub mod cache;

use std::collections::HashSet;

use serde_json::{json, Map, Value};

pub fn build_search_query(
    input: &str,
    from: usize,
    search_after: Option<String>,
) -> Value {
    let query_string = to_simple_qs(input);

    // ---------- static skeleton ------------------------------------------------
    let mut body = json!({
        "_source": ["url", "title", "content"],
        // we’ll insert "from" OR "search_after" below
        "size": 20,
        "track_total_hits": false,

        "query": {
            "simple_query_string": {
                "query":               query_string,
                "fields":              ["title^2", "content"],
                "default_operator":    "and",
                "flags":               "AND|OR|PHRASE|FUZZY",
                "fuzzy_prefix_length": 1,
                "fuzzy_max_expansions": 100
            }
        },

        "highlight": {
            "pre_tags":  ["<b>"],
            "post_tags": ["</b>"],
            "fields": {
                "title":   { "fragment_size": 500, "number_of_fragments": 1 },
                "content": { "fragment_size": 500, "number_of_fragments": 1 },
                "url":     { "number_of_fragments": 0 }
            }
        },

        // deterministic order + cheap tie-breaker
        "sort": [
            { "_score":          { "order": "desc" } },
            { "page_created_at": { "order": "desc" } },
            { "_shard_doc":      { "order": "asc"  } }  // unique per shard
        ]
    });

    // ---------- paging — either "from" OR "search_after" -----------------------
    let obj: &mut Map<String, Value> = body.as_object_mut().unwrap();

    if let Some(cursor) = search_after {
        // caller already has a cursor → use it
        //   1) drop "from"
        //   2) insert parsed JSON array into "search_after"
        obj.insert(
            "search_after".to_string(),
            serde_json::from_str::<Value>(&cursor)
                .unwrap_or_else(|_| json!(null)),
        );
    } else {
        // first page
        obj.insert("from".to_string(), json!(from));
    }

    body
}

fn to_simple_qs(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len() * 2);
    let mut need_and = false;

    for orig in raw.split_whitespace() {
        let token = orig.trim();

        // rule 4 — pass through OR
        if token.eq_ignore_ascii_case("OR") && !is_quoted(token) {
            out.push_str(" OR ");
            need_and = false;
            continue;
        }

        if need_and {
            out.push_str(" AND ");
        }

        let processed = if is_quoted(token) {
            token.to_owned()                                // rule 5
        } else if token.chars().all(|c| c.is_ascii_digit()) {
            format!("\"{}\"", escape_special(token))
        } else {
            format!("\"{}\"", escape_special(token))
        };

        out.push_str(&processed);
        need_and = true;
    }

    out
}

#[inline]
fn is_quoted(s: &str) -> bool {
    s.len() >= 2 && s.starts_with('"') && s.ends_with('"')
}

/// Escape Lucene’s special characters for `simple_query_string`.
fn escape_special(s: &str) -> String {
    // All special characters defined by Lucene for query parsing
    const SPECIAL_CHARS: [char; 22] = [
        '+', '-', '=', '&', '|', '>', '<', '!', '(', ')',
        '{', '}', '[', ']', '^', '"', '~', '*', '?', ':', '\\', '/',
    ];

    let special: HashSet<char> = SPECIAL_CHARS.iter().cloned().collect();
    let mut out = String::with_capacity(s.len());

    for ch in s.chars() {
        if special.contains(&ch) {
            out.push('\\');
        }
        out.push(ch);
    }

    out
}
