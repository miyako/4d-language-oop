// overload 0
var $receiver : Collection
$receiver:=New collection(1; 2; 3)
var $result1 : Collection
$result1:=$receiver.fill("synthText")
// overload 1
$receiver:=New collection(1; 2; 3)
var $result2 : Collection
$result2:=$receiver.fill("synthText"; 1; 1)
// overload 1 [optional:omit-end]
$receiver:=New collection(1; 2; 3)
var $result3 : Collection
$result3:=$receiver.fill("synthText"; 1)
