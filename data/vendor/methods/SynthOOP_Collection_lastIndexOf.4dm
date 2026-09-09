// overload 0
var $receiver : Collection
$receiver:=New collection(1; 2; 3)
var $result1 : Integer
$result1:=$receiver.lastIndexOf(Formula(1+2); 1)
// overload 0 [optional:omit-startFrom]
$receiver:=New collection(1; 2; 3)
var $result2 : Integer
$result2:=$receiver.lastIndexOf(Formula(1+2))
