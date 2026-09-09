// overload 0
var $receiver : Collection
$receiver:=New collection(1; 2; 3)
var $formula1 : 4D.Function
$formula1:=Formula(1+2)
var $result2 : Variant
$result2:=$receiver.reduce($formula1; "synthText"; "synthText")
// overload 1
$receiver:=New collection(1; 2; 3)
var $result3 : Variant
$result3:=$receiver.reduce("synthText"; "synthText"; "synthText")
