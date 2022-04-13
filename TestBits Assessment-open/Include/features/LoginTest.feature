Feature: Login feature

  Scenario Outline: Login test valid
    Given User navigates to login page
    When User enters <username> and <password>
    And Click on login button
    Then User is navigated to homepage
    
  Examples:
  	|username|password|
  	|John Doe|ThisIsNotAPassword|
  	
  Scenario Outline: Login test invalid
    Given User navigates to login page
    When User enters <username> and <password>
    And Click on login button
    Then User invalid password
    
   Examples:
  	|username|password|
  	|John Cena|f3f3f3rddgd|