// overload 0
var $receiver : Collection
$receiver:=New collection(1; 2; 3)
var $result1 : Collection
$result1:=$receiver.combine(New collection; 1)
// overload 0 [optional:omit-index]
$receiver:=New collection(1; 2; 3)
var $result2 : Collection
$result2:=$receiver.combine(New collection)
