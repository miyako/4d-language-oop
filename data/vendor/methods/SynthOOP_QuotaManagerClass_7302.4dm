// property read
var $receiver : 4D.QuotaManagerClass
$receiver:=Session.quotas
var $read1 : Integer
$read1:=$receiver.nbRequestsPerHour
// property write
$receiver:=Session.quotas
$receiver.nbRequestsPerHour:=1
