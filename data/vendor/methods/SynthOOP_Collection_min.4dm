// overload 0
var $receiver : Collection
$receiver:=New collection(1; 2; 3)
var $result1 : Variant
$result1:=$receiver.min("synthText")
// overload 0 [optional:omit-propertyPath]
$receiver:=New collection(1; 2; 3)
var $result2 : Variant
$result2:=$receiver.min()
