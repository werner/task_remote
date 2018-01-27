# Task Remote

## Execute tasks on a Remote linux server

### Quick Start Guide

* Add a Task
  * Command:  ``` sudo su - user -c "cd /var/www/app; bundle exec rails runner $CODE"```
  * Code:  ``` Sale.where(tax: 1.14) ```
  * Server: ``` user@server.com ```