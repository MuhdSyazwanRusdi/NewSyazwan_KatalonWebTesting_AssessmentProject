# Syazwan_KatalonWebTesting_AssessmentProject
 
Five group features and scenarious

*Note: Please Change Execution Profile from 'default' to 'Admin orange HR'

1. Recruitment > Candidates 

a. Add New Candidate 
(+) Successfully adding new candidate 
(-) Users did not enter candidate email 
(-) File selected larger than 1MB 
(-) Users leave the form blank 

b. Delete Candidate 
(+) Successfully delete a candidate 
(+) Users delete more than one candidate (KIV)

2. PIM > Employee List 

a. Add Employee 
(+) Successfully add employee 
(-) Register using existing employee ID 
(-) Selected file is larger than 1MB 
(-) Users enter registered username 
(-) Login password is less than 8 characters 
(+) Login password is very weak 
(-) Password confirmation doesn't match password 

b. Search Employee 
(+) Search employee using ID 
(-) Users enter invalid username 
(+) Search employee using name 
(-) Users enter an unregistered name 
(+) Search employee base on employment status 
(+) Search employee base on job title 
(+) search employee using supervisor name 
(+) Search employee using on sub-unit 

c.Delete Employee Record 
(+) Successfully delete an employee’s record 
(+) Users delete more than one employee’s record (KIV)

3. Admin > Organization > Locations 

a. Add Location 
(+) Successfully add location 
(-) Users enter non-numeric for phone number 
(-) Users enter non-numeric for fax number 

b. Delete Location 
(+) Successfully delete a location 
(+) Users delete more than one location 

c. Edit Location 
(+) Successfully update general information 
(-) User did not fill in organization name 
(-) Users enter non-numeric for phone number 
(-) Users enter non-numeric for fax number 
(+) Users replace name 

4. Admin > Qualifications > Membership 

a. Add New Membership 
(+) Successfully add new membership 
(-) Users enter the same membership 
(-) Users leave the membership name blank 

b. Delete Membership 
(+) Successfully delete a membership 
(+) Users delete more than one membership (KIV)

c. Edit Existing Membership 
(+) Successfully rename the membership 
(-) Users enter the same membership 
(-) Users leave the form blank

5. Recruitment > Vacancies 

a. Add New Vacancy 
(+) Successfully add new vacancy 
(-) Users leave required field blank 

b. Delete Vacancy 
(+) Successfully delete a candidate 
(+) Users delete more than one candidate (KIV)


For Behaviour Driven Development:

1. JobTitle

a. Add JobTitle
(+) Sucessfully add new job title
(-) Users leave required field blank 
(-) Selected file is larger than 1MB 
(-) Users enter registered job title




