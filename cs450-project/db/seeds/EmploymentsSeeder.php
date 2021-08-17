<?php


use Phinx\Seed\AbstractSeed;

class EmploymentsSeeder extends AbstractSeed
{
    public function getDependencies()
    {
        return ['StudentsSeeder', 'UserSeeder'];

    }

    public function run()
    {
        $this->execute('SET FOREIGN_KEY_CHECKS = 0');
        $this->execute('TRUNCATE TABLE tbl_fact_employments');

        $sql = file_get_contents(__DIR__ . '/../sql/015_add_employments.sql');
        $this->execute($sql);
    }
}
