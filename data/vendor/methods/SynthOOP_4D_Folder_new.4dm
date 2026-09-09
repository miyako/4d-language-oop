// overload 0
var $result1 : 4D.Folder
$result1:=4D.Folder.new("synthText"; fk platform path)
// overload 0 [enum 4D.Folder.new.pathType = fk posix path]
var $result2 : 4D.Folder
$result2:=4D.Folder.new("synthText"; fk posix path)
// overload 0 [optional:omit-pathType]
var $result3 : 4D.Folder
$result3:=4D.Folder.new("synthText")
// overload 1
var $result4 : 4D.Folder
$result4:=4D.Folder.new(fk applications folder)
// overload 1 [enum 4D.Folder.new.folderConstant = fk data folder]
var $result5 : 4D.Folder
$result5:=4D.Folder.new(fk data folder)
// overload 1 [enum 4D.Folder.new.folderConstant = fk database folder]
var $result6 : 4D.Folder
$result6:=4D.Folder.new(fk database folder)
// overload 1 [enum 4D.Folder.new.folderConstant = fk desktop folder]
var $result7 : 4D.Folder
$result7:=4D.Folder.new(fk desktop folder)
// overload 1 [enum 4D.Folder.new.folderConstant = fk documents folder]
var $result8 : 4D.Folder
$result8:=4D.Folder.new(fk documents folder)
// overload 1 [enum 4D.Folder.new.folderConstant = fk editor theme folder]
var $result9 : 4D.Folder
$result9:=4D.Folder.new(fk editor theme folder)
// overload 1 [enum 4D.Folder.new.folderConstant = fk home folder]
var $result10 : 4D.Folder
$result10:=4D.Folder.new(fk home folder)
// overload 1 [enum 4D.Folder.new.folderConstant = fk licenses folder]
var $result11 : 4D.Folder
$result11:=4D.Folder.new(fk licenses folder)
// overload 1 [enum 4D.Folder.new.folderConstant = fk logs folder]
var $result12 : 4D.Folder
$result12:=4D.Folder.new(fk logs folder)
// overload 1 [enum 4D.Folder.new.folderConstant = fk mobileApps folder]
var $result13 : 4D.Folder
$result13:=4D.Folder.new(fk mobileApps folder)
// overload 1 [enum 4D.Folder.new.folderConstant = fk remote database folder]
var $result14 : 4D.Folder
$result14:=4D.Folder.new(fk remote database folder)
// overload 1 [enum 4D.Folder.new.folderConstant = fk resources folder]
var $result15 : 4D.Folder
$result15:=4D.Folder.new(fk resources folder)
// overload 1 [enum 4D.Folder.new.folderConstant = fk system folder]
var $result16 : 4D.Folder
$result16:=4D.Folder.new(fk system folder)
// overload 1 [enum 4D.Folder.new.folderConstant = fk user preferences folder]
var $result17 : 4D.Folder
$result17:=4D.Folder.new(fk user preferences folder)
// overload 1 [enum 4D.Folder.new.folderConstant = fk web root folder]
var $result18 : 4D.Folder
$result18:=4D.Folder.new(fk web root folder)
