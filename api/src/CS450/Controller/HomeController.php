<?php

namespace CS450\Controller;

/**
 * @codeCoverageIgnore
 */
class HomeController
{
    public function __invoke($params)
    {
        var_dump($params);
        echo "<div><h1>Hi Dummy! Oh, no, not you! I'm the dummy (page)<h1>";
        phpinfo();
        echo "</div>";
    }
}
