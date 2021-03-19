-- Your SQL goes here
CREATE TABLE IF NOT EXISTS `user` (
   `user_id` INT AUTO_INCREMENT NOT NULL,
   `user_name` VARCHAR(20) NOT NULL,
   `user_password` VARCHAR(40) NOT NULL,
   PRIMARY KEY (`user_id`)
);