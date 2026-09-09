// overload 0
var $receiver : Collection
$receiver:=New collection(1; 2; 3)
var $result1 : Collection
$result1:=$receiver.flat(1)
// overload 0 [optional:omit-depth]
$receiver:=New collection(1; 2; 3)
var $result2 : Collection
$result2:=$receiver.flat()
