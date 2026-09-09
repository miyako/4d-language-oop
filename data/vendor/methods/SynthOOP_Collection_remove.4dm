// overload 0
var $receiver : Collection
$receiver:=New collection(1; 2; 3)
var $result1 : Collection
$result1:=$receiver.remove(1; 1)
// overload 0 [optional:omit-howMany]
$receiver:=New collection(1; 2; 3)
var $result2 : Collection
$result2:=$receiver.remove(1)
