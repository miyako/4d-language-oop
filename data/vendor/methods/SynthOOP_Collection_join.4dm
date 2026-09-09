// overload 0
var $receiver : Collection
$receiver:=New collection(1; 2; 3)
var $result1 : Text
$result1:=$receiver.join("synthText"; ck ignore null or empty)
// overload 0 [optional:omit-option]
$receiver:=New collection(1; 2; 3)
var $result2 : Text
$result2:=$receiver.join("synthText")
