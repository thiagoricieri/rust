/*!
Removes the common level of indention from description strings. For
instance, if an entire doc comment is indented 8 spaces we want to
remove those 8 spaces from every line.

The first line of a string is allowed to be intend less than
subsequent lines in the same paragraph in order to account for
instances where the string containing the doc comment is opened in the
middle of a line, and each of the following lines is indented.
*/

export mk_pass;

fn mk_pass() -> Pass {
    text_pass::mk_pass(~"unindent", unindent)
}

fn unindent(s: ~str) -> ~str {
    let lines = str::lines_any(s);
    let mut saw_first_line = false;
    let mut saw_second_line = false;
    let min_indent = do vec::foldl(uint::max_value, lines)
        |min_indent, line| {

        // After we see the first non-whitespace line, look at
        // the line we have. If it is not whitespace, and therefore
        // part of the first paragraph, then ignore the indentation
        // level of the first line
        let ignore_previous_indents =
            saw_first_line &&
            !saw_second_line &&
            !str::is_whitespace(line);

        let min_indent = if ignore_previous_indents {
            uint::max_value
        } else {
            min_indent
        };

        if saw_first_line {
            saw_second_line = true;
        }

        if str::is_whitespace(line) {
            min_indent
        } else {
            saw_first_line = true;
            let mut spaces = 0;
            do str::all(line) |char| {
                // Only comparing against space because I wouldn't
                // know what to do with mixed whitespace chars
                if char == ' ' {
                    spaces += 1;
                    true
                } else {
                    false
                }
            };
            uint::min(min_indent, spaces)
        }
    };

    if vec::is_not_empty(lines) {
        let unindented = ~[str::trim(vec::head(lines))]
            + do par::map(vec::tail(lines)) |line| {
            if str::is_whitespace(line) {
                line
            } else {
                assert str::len(line) >= min_indent;
                str::slice(line, min_indent, str::len(line))
            }
        };
        str::connect(unindented, ~"\n")
    } else {
        s
    }
}

#[test]
fn should_unindent() {
    let s = ~"    line1\n    line2";
    let r = unindent(s);
    assert r == ~"line1\nline2";
}

#[test]
fn should_unindent_multiple_paragraphs() {
    let s = ~"    line1\n\n    line2";
    let r = unindent(s);
    assert r == ~"line1\n\nline2";
}

#[test]
fn should_leave_multiple_indent_levels() {
    // Line 2 is indented another level beyond the
    // base indentation and should be preserved
    let s = ~"    line1\n\n        line2";
    let r = unindent(s);
    assert r == ~"line1\n\n    line2";
}

#[test]
fn should_ignore_first_line_indent() {
    // Thi first line of the first paragraph may not be indented as
    // far due to the way the doc string was written:
    //
    // #[doc = "Start way over here
    //          and continue here"]
    let s = ~"line1\n    line2";
    let r = unindent(s);
    assert r == ~"line1\nline2";
}

#[test]
fn should_not_ignore_first_line_indent_in_a_single_line_para() {
    let s = ~"line1\n\n    line2";
    let r = unindent(s);
    assert r == ~"line1\n\n    line2";
}
