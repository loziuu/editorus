# Features
[x] Remove - backspace

[x] Remove line with backspace on last position

[x] Move rest of the line to new line with Enter

[x] On moving cursor to left - move to last position in previous line

[x] On moving cursor to right - move to first position in next line

[x] Don't allow cursor to go out of bounds in ERow

[x] Use anything else other than vector ugh...

[x] Separate display from data buffer (Rope)

[x] New line on > 1 byte character does not work

[] Viewport

[] Move viewport on cursor move right

[] Move viewport on cursor move left

[] Line wrapping

[] Handle delete key

[] Command for edit mode

[] "Dirty" on ERow level, (is this even still relevant?)

[] Handle unicode (<- it's on rope level i guess?)

[] Save file

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

[] Log to files :)

[] Refactor cursor.rs. Logic is in two places now dude... c'mon!

[] Add tests?

[] Handle args

[] Add delete key handler

[] Line numbers width should be dynamic (based on number of lines)

# Known issues
-- Too many to list :( 

[] Fix going up and down on non-zero offset-x of vieport
