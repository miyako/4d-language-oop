// property read
var $receiver : 4D.QuotaManagerClass
$receiver:=Session.quotas
var $read1 : Integer
$read1:=$receiver.defaultEntitySetTimeout
// property write
$receiver:=Session.quotas
$receiver.defaultEntitySetTimeout:=1
