<?php


use Phinx\Seed\AbstractSeed;

class PurchasesSeeder extends AbstractSeed
{
    public function getDependencies()
    {
        return ['GrantsSeeder', 'UserSeeder'];
    }
    /**
     * Run Method.
     *
     * Write your database seeder using this method.
     *
     * More information on writing seeders is available here:
     * https://book.cakephp.org/phinx/0/en/seeding.html
     */
    public function run()
    {
        $this->execute('SET FOREIGN_KEY_CHECKS = 0');
        $this->execute('TRUNCATE TABLE tbl_fact_purchases');

        $sql = file_get_contents(__DIR__ . '/../sql/019_add_purchases.sql');
        $this->execute($sql);
    }
}
