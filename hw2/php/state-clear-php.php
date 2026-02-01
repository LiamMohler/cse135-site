<?php
session_start();
session_destroy();
header("Location: state-php.php");
exit();
?>