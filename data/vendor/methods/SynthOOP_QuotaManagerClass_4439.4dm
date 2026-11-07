// property read
var $receiver : 4D.QuotaManagerClass
$receiver:=Session.quotas
var $read1 : Integer
$read1:=$receiver.inBytesPerMin
// property write
$receiver:=Session.quotas
$receiver.inBytesPerMin:=1
