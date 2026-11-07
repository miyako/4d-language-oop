// property read
var $receiver : 4D.QuotaManagerClass
$receiver:=Session.quotas
var $read1 : Integer
$read1:=$receiver.nbGuestSessions
// property write
$receiver:=Session.quotas
$receiver.nbGuestSessions:=1
