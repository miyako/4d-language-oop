// property read
var $receiver : 4D.MailAttachment
$receiver:=4D.MailAttachment.new(File("/PACKAGE/report.pdf"))
var $read1 : Integer
$read1:=$receiver.size
// property write
$receiver:=4D.MailAttachment.new(File("/PACKAGE/report.pdf"))
$receiver.size:=1
