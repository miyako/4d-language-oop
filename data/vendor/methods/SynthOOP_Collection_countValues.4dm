// overload 0
var $receiver : Collection
$receiver:=New collection(1; 2; 3)
var $result1 : Real
$result1:=$receiver.countValues("synthText"; "synthText")
// overload 0 [optional:omit-propertyPath]
$receiver:=New collection(1; 2; 3)
var $result2 : Real
$result2:=$receiver.countValues("synthText")
