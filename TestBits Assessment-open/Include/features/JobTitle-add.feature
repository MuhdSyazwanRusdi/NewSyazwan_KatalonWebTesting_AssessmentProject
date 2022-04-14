Feature: Job Title Adding

  Scenario Outline: Positive_Success adding
    Given Login until job title
    When User enter <jobname> and <description>
    And User click save
    Then User is navigated to jobtitle
	Examples:
  	|jobname|description|
  	|Singer|Entertain colleague|
  	
  	
  Scenario: Negative_Fill blank
    Given Login until job title
    And User click save
    Then Jobtitle is required
	

  Scenario Outline: Negative_File Upload exceed limit
    Given Login until job title
    When User enter <jobname> and <description>
    And User add file and click save
    Then File exceed 1mb
	Examples:
  	|jobname|description|
  	|Singer2|Entertain colleague|
  	
  	
  
  Scenario Outline: Negative_Same Job Title
    Given Login until job title
    When User enter <jobname> and <description>
    And User click save
    Then Jobtitle is used before
	Examples:
  	|jobname|description|
  	|Singer|Entertain colleague|