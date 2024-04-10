# Features
[x] Remove - backspace

[x] Remove line with backspace on last position

[x] Move rest of the line to new line with Enter

[x] On moving cursor to left - move to last position in previous line

[x] On moving cursor to right - move to first position in next line

[x] Don't allow cursor to go out of bounds in ERow

[x] Use anything else other than vector ugh...

[x] Separate display from data buffer (Rope)

[x] Viewport

[x] Handle delete key

[x] Handle backspace key

[x] Handle utf-8

[x] Save file

[] "Dirty" on ERow level, (is this even still relevant?)

[] Add command

[] Add different modes (at least motion - edit)?

[] Undo

[] Redo

[] Breaking line on rows with more characters than cols (display buffer)

[] Formatting?

[] Styles

[] Find

[] Selection

[] Viewports

[] Add tests?

[] Handle args

[] Line numbers width should be dynamic (based on number of lines)

# Known issues
[] Something brakes if I [delete] from line that contains utf-8 character
