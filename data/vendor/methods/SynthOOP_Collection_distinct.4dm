// overload 0
var $receiver : Collection
$receiver:=New collection(1; 2; 3)
var $result1 : Collection
$result1:=$receiver.distinct(ck diacritical)
// overload 0 [enum Collection.distinct.options = ck count values]
$receiver:=New collection(1; 2; 3)
var $result2 : Collection
$result2:=$receiver.distinct(ck count values)
// overload 0 [optional:omit-options]
$receiver:=New collection(1; 2; 3)
var $result3 : Collection
$result3:=$receiver.distinct()
// overload 1
$receiver:=New collection(1; 2; 3)
var $result4 : Collection
$result4:=$receiver.distinct("synthText"; ck diacritical)
// overload 1 [enum Collection.distinct.options = ck count values]
$receiver:=New collection(1; 2; 3)
var $result5 : Collection
$result5:=$receiver.distinct("synthText"; ck count values)
// overload 1 [optional:omit-options]
$receiver:=New collection(1; 2; 3)
var $result6 : Collection
$result6:=$receiver.distinct("synthText")
