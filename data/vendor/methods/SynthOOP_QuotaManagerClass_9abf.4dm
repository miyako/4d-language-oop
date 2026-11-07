// property read
var $receiver : 4D.QuotaManagerClass
$receiver:=Session.quotas
var $read1 : Integer
$read1:=$receiver.nbEntitySets
// property write
$receiver:=Session.quotas
$receiver.nbEntitySets:=1
