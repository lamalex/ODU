<?php

namespace CS450\Service\Email;

use Mailgun\Mailgun;
use CS450\Lib\EmailAddress;
use CS450\Service\EmailService;

final class MailgunEmail implements EmailService {
    
    /**
     *
     * @Inject("mailgun.cfg")
     */
    private $cfg;

    /**
     *
     * @Inject
     * @var \Psr\Log\LoggerInterface
     */
    private $logger;

    public function sendFromTemplate(EmailAddress $to, string $subject, string $template, $data) {
        $this->logger->info("Emailing " . $to . " with subject " . $subject . " and " . json_encode($data));

        $mg = Mailgun::create($this->cfg->apiKey);
        $res = $mg->messages()->send($this->cfg->domain, [
            'from'     => "mailgun@" . $this->cfg->domain,
            'to'       => $to,
            'subject'  => $subject,
            'text'     => $body,
            'template' => $template,
            'h:X-Mailgun-Variables' => json_encode($data),
        ]);
    }
}
