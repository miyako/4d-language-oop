// overload 0
var $receiver : Collection
$receiver:=New collection(1; 2; 3)
var $result1 : Collection
$result1:=$receiver.copy()
// overload 1
$receiver:=New collection(1; 2; 3)
var $result2 : Collection
$result2:=$receiver.copy(ck resolve pointers)
// overload 1 [enum Collection.copy.option = ck shared]
$receiver:=New collection(1; 2; 3)
var $result3 : Collection
$result3:=$receiver.copy(ck shared)
// overload 2
$receiver:=New collection(1; 2; 3)
var $result4 : Collection
$result4:=$receiver.copy(ck resolve pointers; New collection)
// overload 2 [enum Collection.copy.option = ck shared]
$receiver:=New collection(1; 2; 3)
var $result5 : Collection
$result5:=$receiver.copy(ck shared; New collection)
// overload 3
$receiver:=New collection(1; 2; 3)
var $result6 : Collection
$result6:=$receiver.copy(ck resolve pointers; New object)
// overload 3 [enum Collection.copy.option = ck shared]
$receiver:=New collection(1; 2; 3)
var $result7 : Collection
$result7:=$receiver.copy(ck shared; New object)
