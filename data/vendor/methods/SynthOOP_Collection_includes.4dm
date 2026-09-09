// overload 0
var $receiver : Collection
$receiver:=New collection(1; 2; 3)
var $result1 : Boolean
$result1:=$receiver.includes(Formula(1+2); 1)
// overload 0 [optional:omit-startFrom]
$receiver:=New collection(1; 2; 3)
var $result2 : Boolean
$result2:=$receiver.includes(Formula(1+2))
