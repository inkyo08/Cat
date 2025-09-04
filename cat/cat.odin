package cat

import "core:c"

@(export, link_name="Start")
Start :: proc "c" (mode: int) -> int {

   return 0;

}
