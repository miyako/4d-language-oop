// overload 0
var $receiver : Collection
$receiver:=New collection(1; 2; 3)
var $result1 : Collection
$result1:=$receiver.extract("synthText"; ck keep null)
// overload 0 [optional:omit-option]
$receiver:=New collection(1; 2; 3)
var $result2 : Collection
$result2:=$receiver.extract("synthText")
// overload 1
$receiver:=New collection(1; 2; 3)
var $result3 : Collection
$result3:=$receiver.extract("synthText"; "synthText"; "synthText")
