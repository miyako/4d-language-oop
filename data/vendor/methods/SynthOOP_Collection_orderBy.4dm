// overload 0
var $receiver : Collection
$receiver:=New collection(1; 2; 3)
var $result1 : Collection
$result1:=$receiver.orderBy()
// overload 1
$receiver:=New collection(1; 2; 3)
var $result2 : Collection
$result2:=$receiver.orderBy("synthText")
// overload 2
$receiver:=New collection(1; 2; 3)
var $result3 : Collection
$result3:=$receiver.orderBy(New collection)
// overload 3
$receiver:=New collection(1; 2; 3)
var $result4 : Collection
$result4:=$receiver.orderBy(ck ascending)
// overload 3 [enum Collection.orderBy.ascOrDesc = ck descending]
$receiver:=New collection(1; 2; 3)
var $result5 : Collection
$result5:=$receiver.orderBy(ck descending)
