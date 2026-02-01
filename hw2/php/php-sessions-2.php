<?php
// Start session
session_start();

// Get username from session
$username = isset($_SESSION['username']) ? $_SESSION['username'] : '';

// Send headers
header("Content-Type: text/html");
?>
<!DOCTYPE html>
<html>
<head>
    <title>PHP Sessions Page 2</title>
</head>
<body>
    <h1>PHP Sessions Page 2</h1>
    
    <?php if ($username): ?>
        <p><b>Name:</b> <?php echo htmlspecialchars($username); ?></p>
    <?php else: ?>
        <p><b>Name:</b> You do not have a name set</p>
    <?php endif; ?>
    
    <br><br>
    <a href="php-sessions-1.php">Session Page 1</a><br>
    <a href="php-cgiform.html">PHP CGI Form</a><br>
    <form style="margin-top:30px" action="php-destroy-session.php" method="get">
        <button type="submit">Destroy Session</button>
    </form>
</body>
</html>
