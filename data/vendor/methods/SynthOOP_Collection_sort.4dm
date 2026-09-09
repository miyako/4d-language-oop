// overload 0
var $receiver : Collection
$receiver:=New collection(1; 2; 3)
var $result1 : Collection
$result1:=$receiver.sort()
// overload 1
$receiver:=New collection(1; 2; 3)
var $result2 : Collection
$result2:=$receiver.sort(ck ascending)
// overload 1 [enum Collection.sort.ascOrDesc = ck descending]
$receiver:=New collection(1; 2; 3)
var $result3 : Collection
$result3:=$receiver.sort(ck descending)
// overload 2
$receiver:=New collection(1; 2; 3)
var $formula4 : 4D.Function
$formula4:=Formula(1+2)
var $result5 : Collection
$result5:=$receiver.sort($formula4; "synthText")
// overload 3
$receiver:=New collection(1; 2; 3)
var $result6 : Collection
$result6:=$receiver.sort("synthText"; "synthText")
