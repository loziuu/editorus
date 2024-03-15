# Features
[x] Remove - backspace
[x] Remove line with backspace on last position
[x] Move rest of the line to new line with Enter
[x] On moving cursor to left - move to last position in previous line
[x] On moving cursor to right - move to first position in next line
[] Command for edit mode
[] "Dirty" on ERow level, (is this even still relevant?)
[] Handle unicode (<- it's on rope level i guess?)
[] Don't allow cursor to go out of bounds in ERow
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
[~] Use anything else other than vector ugh...
    - Rope is not that far away from being usable
[] Refactor cursor.rs. Logic is in two places now dude... c'mon!
[] Add tests?
[] Separate display from data buffer (Rope)

# Known issues
