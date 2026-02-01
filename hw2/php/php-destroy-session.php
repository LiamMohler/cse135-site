#!/usr/bin/php
<?php
// Start session
session_start();

// Destroy session
session_destroy();

// Send headers
header("Content-Type: text/html");
?>
<!DOCTYPE html>
<html>
<head>
    <title>Session Destroyed</title>
</head>
<body>
    <h1>Session Destroyed</h1>
    <p>Your session has been cleared.</p>
    <a href="php-cgiform.html">Back to the PHP CGI Form</a><br>
    <a href="php-sessions-1.php">Back to Page 1</a><br>
    <a href="php-sessions-2.php">Back to Page 2</a>
</body>
</html>
