// property read
var $receiver : 4D.MailAttachment
$receiver:=4D.MailAttachment.new(File("/PACKAGE/report.pdf"))
var $read1 : Text
$read1:=$receiver.name
// property write
$receiver:=4D.MailAttachment.new(File("/PACKAGE/report.pdf"))
$receiver.name:="synthText"
