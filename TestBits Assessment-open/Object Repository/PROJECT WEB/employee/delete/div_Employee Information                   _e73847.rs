<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Employee Information                   _e73847</name>
   <tag></tag>
   <elementGuidId>8b9c14cc-07b0-4f7c-866b-41dc99371781</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>#content</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='content']</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>c7d42265-cd43-4718-b749-43cfff3cefc5</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>content</value>
      <webElementGuid>16377cc9-694c-4161-abb7-603422fd03ff</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>

                  

    
        Employee Information
    
    
        



        

            

                
                    Employee Name
  





        

            var employees_empsearch_employee_name = [];

            $(document).ready(function() {
            
                var nameField = $(&quot;#empsearch_employee_name_empName&quot;);
                var idStoreField = $(&quot;#empsearch_employee_name_empId&quot;);
                var typeHint = 'Type for hints...';
                var hintClass = 'inputFormatHint';
                var loadingMethod = 'ajax';
                var loadingHint = 'Loading';
            
                if (idStoreField.val() != '') {
                    idStoreField.data('item.name', nameField.val());
                }
                
                nameField.data('typeHint', typeHint);
                nameField.data('loadingHint', loadingHint);
                
                nameField.one('focus', function() {

                        if ($(this).hasClass(hintClass)) {
                            $(this).val(&quot;&quot;);
                            $(this).removeClass(hintClass);
                        }

                    });
                    
                if( loadingMethod != 'ajax'){
                    if (nameField.val() == '' || nameField.val() == typeHint) {
                        nameField.val(typeHint).addClass(hintClass);
                    }

                    

                    nameField.autocomplete(employees_empsearch_employee_name, {

                        formatItem: function(item) {
                            return $('&lt;div/>').text(item.name).html();
                        },
                        formatResult: function(item) {
                            return item.name
                        }
                      ,matchContains:true
                        }).result(function(event, item) {
                            idStoreField.val(item.id);
                            idStoreField.data('item.name', item.name);
                        }

                    );
                 }else{
                        var value = nameField.val().trim();
                        nameField.val(loadingHint).addClass('ac_loading');
                        $.ajax({
                               url: &quot;/index.php/pim/getEmployeeListAjax&quot;,
                               data: '',
                               dataType: 'json',
                               success: function(employeeList){

                                     nameField.autocomplete(employeeList, {

                                                formatItem: function(item) {
                                                    return $('&lt;div/>').text(item.name).html();
                                                },
                                                formatResult: function(item) {
                                                    return item.name
                                                }
                                                
                                                ,matchContains:true
                                            }).result(function(event, item) {
                                                idStoreField.val(item.id);
                                                idStoreField.data('item.name', item.name);
                                            }

                                        );
                                         nameField.removeClass('ac_loading'); 
                                        
                                         if(value==''){
                                            nameField.val(typeHint).addClass(hintClass);
                                         } else {
                                            nameField.val(value).addClass();
                                         }
                                    }
                             });
                 }
                
            }); // End of $(document).ready

                 
        



Id
  

Employment Status
  
All
Freelance
Full-Time Contract
Full-Time Permanent
Full-Time Probation
Part-Time Contract
Part-Time Internship


Include
  
Current Employees Only
Current and Past Employees
Past Employees Only


Supervisor Name
  

Job Title
  
All
Automation Tester
BTest
Chief Executive Officer
Chief Financial Officer
Content Specialist
Customer Success Manager
Database Administrator
EA
Engineer
Finance Manager
Financial Analyst
Head of Support
HR Associate
HR Manager
IT Manager
Network Administrator
Payroll Administrator
Phó phòng đào tạo
Pre-Sales Coordinator
QA Engineer
QA Lead
Sales Representative
Social Media Marketer
Software Architect
Software Engineer
Support Specialist
VP - Client Services
VP - Sales &amp; Marketing


Sub Unit
  
All
Administration
Engineering
  Development
  Quality Assurance
  TechOps
Sales &amp; Marketing
  Sales
  Marketing
Client Services
  Technical Support
Finance
Human Resources



                

                
                                 

                
                    
                                        
                

            

        

     

    >

 


    
        
    
    

                
            
    
                
           
        

        


 1-50 of 62 First Previous 1 2 Next Last  

         

        


         
        

        
            
            
                
        
        
            
                                
                
Id
First (&amp; Middle) Name
Last Name
Job Title
Employment Status
Sub Unit
Supervisor
            
                            

            
                                            
                                                    0272
                                                    Admin
                                                    A
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0273
                                                    Admin
                                                    A
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0345
                                                    Odis
                                                    Adalwin
                                                    Engineer
                                                    Full-Time Permanent
                                                    Administration
                                                    John  Smith
                                            
                                            
                                                    0007
                                                    Peter Mac
                                                    Anderson
                                                    Chief Financial Officer
                                                    Full-Time Probation
                                                    Administration
                                                    John  Smith
                                            
                                            
                                                    0016
                                                    Linda Jane
                                                    Anderson
                                                    VP - Sales &amp; Marketing
                                                    Full-Time Permanent
                                                    Administration
                                                    John  Smith
                                            
                                            
                                                    0208
                                                    Lisa
                                                    Andrews
                                                    Software Engineer
                                                    Full-Time Probation
                                                    Development
                                                    Fiona  Grace
                                            
                                            
                                                    0247
                                                    Kallyani
                                                    Bhute
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0204
                                                    Cecil
                                                    Bonaparte
                                                    Software Engineer
                                                    Full-Time Permanent
                                                    Development
                                                    Fiona  Grace
                                            
                                            
                                                    0212
                                                    Charlie
                                                    Carter
                                                    QA Engineer
                                                    Full-Time Probation
                                                    Quality Assurance
                                                    Aaliyah  Haq
                                            
                                            
                                                    0020
                                                    Dominic
                                                    Chase
                                                    VP - Client Services
                                                    Full-Time Permanent
                                                    Administration
                                                    John  Smith
                                            
                                            
                                                    0217
                                                    Chenzira
                                                    Chuki
                                                    QA Engineer
                                                    Full-Time Permanent
                                                    Quality Assurance
                                                    Aaliyah  Haq
                                            
                                            
                                                    6117
                                                    Paul
                                                    Collings
                                                    HR Manager
                                                    Full-Time Permanent
                                                    Human Resources
                                                    John  Smith
                                            
                                            
                                                    0251
                                                    Ananya
                                                    Dash
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0253
                                                    Inba Venba
                                                    Dhiya
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0258
                                                    Inba Venba
                                                    Dhiya
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0250
                                                    Prajakta
                                                    Dhumal
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0221
                                                    Alice
                                                    Duval
                                                    Account Assistant (Deleted)
                                                    Full-Time Contract
                                                    Finance
                                                    Kevin  Mathews
                                            
                                            
                                                    0227
                                                    Ehioze
                                                    Ebo
                                                    Account Assistant (Deleted)
                                                    Full-Time Probation
                                                    Finance
                                                    Kevin  Mathews
                                            
                                            
                                                    0231
                                                    Nathan
                                                    Elliot
                                                    Sales Representative
                                                    Full-Time Probation
                                                    Sales
                                                    Linda  Jane  Anderson
                                            
                                            
                                                    0235
                                                    Goutam
                                                    Ganesh
                                                    Sales Representative
                                                    Full-Time Permanent
                                                    Sales
                                                    Linda  Jane  Anderson
                                            
                                            
                                                    0029
                                                    Fiona
                                                    Grace
                                                    Software Architect
                                                    Full-Time Permanent
                                                    Development
                                                    Odis  Adalwin
                                            
                                            
                                                    0034
                                                    Russel
                                                    Hamilton
                                                    Software Engineer
                                                    Full-Time Permanent
                                                    Development
                                                    Fiona  Grace
                                            
                                            
                                                    0038
                                                    Aaliyah
                                                    Haq
                                                    QA Lead
                                                    Full-Time Permanent
                                                    Quality Assurance
                                                    Odis  Adalwin
                                            
                                            
                                                    0042
                                                    Rebecca
                                                    Harmony
                                                    QA Engineer
                                                    Full-Time Contract
                                                    Quality Assurance
                                                    Aaliyah  Haq
                                            
                                            
                                                    0046
                                                    Cassidy
                                                    Hope
                                                    IT Manager
                                                    Part-Time Contract
                                                    TechOps
                                                    Odis  Adalwin
                                            
                                            
                                                    0239
                                                    Kiyara
                                                    Hu
                                                    HR Associate
                                                    Full-Time Permanent
                                                    Human Resources
                                                    Paul  Collings
                                            
                                            
                                                    0243
                                                    Jadine
                                                    Jackie
                                                    HR Associate
                                                    Full-Time Probation
                                                    Human Resources
                                                    Paul  Collings
                                            
                                            
                                                    0278
                                                    Harry
                                                    Kane
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    1289
                                                    Ali Mohd
                                                    Khan
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0248
                                                    Manali
                                                    Kulkarni
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0264
                                                    jyothi
                                                    lakshmisetty
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0262
                                                    sravani
                                                    mallikarjun
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0265
                                                    sravani
                                                    mallikarjun
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0267
                                                    sravani
                                                    mallikarjun
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0269
                                                    sravani
                                                    mallikarjun
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0271
                                                    sravani
                                                    mallikarjun
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0279
                                                    sravani
                                                    mallikarjun
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0050
                                                    Maggie
                                                    Manning
                                                    Network Administrator
                                                    Full-Time Probation
                                                    TechOps
                                                    Cassidy  Hope
                                            
                                            
                                                    0054
                                                    Jordan
                                                    Mathews
                                                    Database Administrator
                                                    Full-Time Contract
                                                    TechOps
                                                    Cassidy  Hope
                                            
                                            
                                                    0058
                                                    Kevin
                                                    Mathews
                                                    Finance Manager
                                                    Full-Time Permanent
                                                    Finance
                                                    Peter  Mac  Anderson
                                            
                                            
                                                    0275
                                                    bhakti sai
                                                    more
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0280
                                                    bhakti sai
                                                    more
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0062
                                                    Jasmine
                                                    Morgan
                                                    Financial Analyst
                                                    Full-Time Probation
                                                    Finance
                                                    Kevin  Mathews
                                            
                                            
                                                    0066
                                                    David
                                                    Morris
                                                    Account Assistant (Deleted)
                                                    Full-Time Permanent
                                                    Finance
                                                    Kevin  Mathews
                                            
                                            
                                                    0255
                                                    La Al
                                                    Na
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0070
                                                    Anthony
                                                    Nolan
                                                    Sales Representative
                                                    Full-Time Probation
                                                    Sales
                                                    Linda  Jane  Anderson
                                            
                                            
                                                    0074
                                                    Nina
                                                    Patel
                                                    Pre-Sales Coordinator
                                                    Full-Time Contract
                                                    Sales
                                                    Anthony  Nolan
                                            
                                            
                                                    0078
                                                    Melan
                                                    Peiris
                                                    Social Media Marketer
                                                    Full-Time Permanent
                                                    Marketing
                                                    Linda  Jane  Anderson
                                            
                                            
                                                    0260
                                                    Varmaa
                                                    Rajeshh
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0277
                                                    Varmaa
                                                    Rajeshh
                                                    
                                                    
                                                    
                                                    
                                            
                                
                            
         
                    
                                
                1-50 of 62 First Previous 1 2 Next Last            
                  
                
        
     
        
 




                    

                        var rootPath = '/';

                        $(document).ready(function() {
                            ohrmList_init();

                        });
                                              





  
    ×
    OrangeHRM - Confirmation Required
  
  
    Delete records?
  
  
    
    
  




    $(document).ready(function() {
        
        var supervisors = [{&quot;name&quot;:&quot;Odis Adalwin&quot;,&quot;id&quot;:&quot;2&quot;},{&quot;name&quot;:&quot;Linda Jane Anderson&quot;,&quot;id&quot;:&quot;5&quot;},{&quot;name&quot;:&quot;Peter Mac Anderson&quot;,&quot;id&quot;:&quot;3&quot;},{&quot;name&quot;:&quot;Dominic Chase&quot;,&quot;id&quot;:&quot;6&quot;},{&quot;name&quot;:&quot;Paul Collings&quot;,&quot;id&quot;:&quot;7&quot;},{&quot;name&quot;:&quot;Fiona Grace&quot;,&quot;id&quot;:&quot;8&quot;},{&quot;name&quot;:&quot;Aaliyah Haq&quot;,&quot;id&quot;:&quot;10&quot;},{&quot;name&quot;:&quot;Cassidy Hope&quot;,&quot;id&quot;:&quot;12&quot;},{&quot;name&quot;:&quot;Kevin Mathews&quot;,&quot;id&quot;:&quot;15&quot;},{&quot;name&quot;:&quot;Anthony Nolan&quot;,&quot;id&quot;:&quot;18&quot;},{&quot;name&quot;:&quot;Melan Peiris&quot;,&quot;id&quot;:&quot;20&quot;},{&quot;name&quot;:&quot;DilshadS S&quot;,&quot;id&quot;:&quot;43&quot;},{&quot;name&quot;:&quot;John Smith&quot;,&quot;id&quot;:&quot;4&quot;}];

        $('#btnDelete').attr('disabled', 'disabled');

        $(&quot;#ohrmList_chkSelectAll&quot;).click(function() {
            if ($(&quot;:checkbox&quot;).length == 1) {
                $('#btnDelete').attr('disabled', 'disabled');
            }
            else {
                if ($(&quot;#ohrmList_chkSelectAll&quot;).is(':checked')) {
                    $('#btnDelete').removeAttr('disabled');
                } else {
                    $('#btnDelete').attr('disabled', 'disabled');
                }
            }
        });

        $(':checkbox[name*=&quot;chkSelectRow[]&quot;]').click(function() {
            if ($(':checkbox[name*=&quot;chkSelectRow[]&quot;]').is(':checked')) {
                $('#btnDelete').removeAttr('disabled');
            } else {
                $('#btnDelete').attr('disabled', 'disabled');
            }
        });

        // Handle hints
        if ($(&quot;#empsearch_id&quot;).val() == '') {
            $(&quot;#empsearch_id&quot;).val('Type Employee Id...')
                    .addClass(&quot;inputFormatHint&quot;);
        }

        if ($(&quot;#empsearch_supervisor_name&quot;).val() == '') {
            $(&quot;#empsearch_supervisor_name&quot;).val('Type for hints...')
                    .addClass(&quot;inputFormatHint&quot;);
        }

        $(&quot;#empsearch_id, #empsearch_supervisor_name&quot;).one('focus', function() {

            if ($(this).hasClass(&quot;inputFormatHint&quot;)) {
                $(this).val(&quot;&quot;);
                $(this).removeClass(&quot;inputFormatHint&quot;);
            }
        });

        $(&quot;#empsearch_supervisor_name&quot;).autocomplete(supervisors, {
            formatItem: function(item) {
                return $('&lt;div/>').text(item.name).html();
            },
            formatResult: function(item) {
                return item.name
            }
            , matchContains: true
        }).result(function(event, item) {
        }
        );

        $('#searchBtn').click(function() {
            $(&quot;#empsearch_isSubmitted&quot;).val('yes');
            $('#search_form input.inputFormatHint').val('');
            $('#search_form input.ac_loading').val('');
            $('#search_form').submit();
        });

        $('#resetBtn').click(function() {
            $(&quot;#empsearch_isSubmitted&quot;).val('yes');
            $(&quot;#empsearch_employee_name_empName&quot;).val('');
            $(&quot;#empsearch_supervisor_name&quot;).val('');
            $(&quot;#empsearch_id&quot;).val('');
            $(&quot;#empsearch_job_title&quot;).val('0');
            $(&quot;#empsearch_employee_status&quot;).val('0');
            $(&quot;#empsearch_sub_unit&quot;).val('0');
            $(&quot;#empsearch_termination&quot;).val('1');
            $('#search_form').submit();
        });

        $('#btnAdd').click(function() {
            location.href = &quot;/index.php/pim/addEmployee&quot;;
        });
        $('#btnDelete').click(function() {
            $('#frmList_ohrmListComponent').submit(function() {
                $('#deleteConfirmation').dialog('open');
                return false;
            });
        });

        /* Delete confirmation controls: Begin */
        $('#dialogDeleteBtn').click(function() {
            document.frmList_ohrmListComponent.submit();
        });
        /* Delete confirmation controls: End */

    }); //ready

    function submitPage(pageNo) {
        document.frmEmployeeSearch.pageNo.value = pageNo;
        document.frmEmployeeSearch.hdnAction.value = 'paging';
        $('#search_form input.inputFormatHint').val('');
        $('#search_form input.ac_loading').val('');
        $(&quot;#empsearch_isSubmitted&quot;).val('no');
        document.getElementById('search_form').submit();
    }


            </value>
      <webElementGuid>00f08697-352b-4599-865f-eb540d70ff82</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;content&quot;)</value>
      <webElementGuid>6ea86cc7-3e3a-455e-8e2c-cbf6109403b7</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//div[@id='content']</value>
      <webElementGuid>0fc57c9e-a6db-4015-8e7c-3c8c6cf38c71</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='wrapper']/div[3]</value>
      <webElementGuid>d443c656-7149-4b14-a374-54bab44488f7</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Buzz'])[1]/following::div[1]</value>
      <webElementGuid>e63a9763-e113-4d00-8f64-7dd3141540b7</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Access Records'])[1]/following::div[1]</value>
      <webElementGuid>ce1e1548-e130-44dc-bdc0-acb75c062a16</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div/div[3]</value>
      <webElementGuid>af986712-d8db-46d7-a3fd-0331b9929d4b</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[@id = 'content' and (text() = concat(&quot;

                  

    
        Employee Information
    
    
        



        

            

                
                    Employee Name
  





        

            var employees_empsearch_employee_name = [];

            $(document).ready(function() {
            
                var nameField = $(&quot;#empsearch_employee_name_empName&quot;);
                var idStoreField = $(&quot;#empsearch_employee_name_empId&quot;);
                var typeHint = &quot; , &quot;'&quot; , &quot;Type for hints...&quot; , &quot;'&quot; , &quot;;
                var hintClass = &quot; , &quot;'&quot; , &quot;inputFormatHint&quot; , &quot;'&quot; , &quot;;
                var loadingMethod = &quot; , &quot;'&quot; , &quot;ajax&quot; , &quot;'&quot; , &quot;;
                var loadingHint = &quot; , &quot;'&quot; , &quot;Loading&quot; , &quot;'&quot; , &quot;;
            
                if (idStoreField.val() != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
                    idStoreField.data(&quot; , &quot;'&quot; , &quot;item.name&quot; , &quot;'&quot; , &quot;, nameField.val());
                }
                
                nameField.data(&quot; , &quot;'&quot; , &quot;typeHint&quot; , &quot;'&quot; , &quot;, typeHint);
                nameField.data(&quot; , &quot;'&quot; , &quot;loadingHint&quot; , &quot;'&quot; , &quot;, loadingHint);
                
                nameField.one(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, function() {

                        if ($(this).hasClass(hintClass)) {
                            $(this).val(&quot;&quot;);
                            $(this).removeClass(hintClass);
                        }

                    });
                    
                if( loadingMethod != &quot; , &quot;'&quot; , &quot;ajax&quot; , &quot;'&quot; , &quot;){
                    if (nameField.val() == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; || nameField.val() == typeHint) {
                        nameField.val(typeHint).addClass(hintClass);
                    }

                    

                    nameField.autocomplete(employees_empsearch_employee_name, {

                        formatItem: function(item) {
                            return $(&quot; , &quot;'&quot; , &quot;&lt;div/>&quot; , &quot;'&quot; , &quot;).text(item.name).html();
                        },
                        formatResult: function(item) {
                            return item.name
                        }
                      ,matchContains:true
                        }).result(function(event, item) {
                            idStoreField.val(item.id);
                            idStoreField.data(&quot; , &quot;'&quot; , &quot;item.name&quot; , &quot;'&quot; , &quot;, item.name);
                        }

                    );
                 }else{
                        var value = nameField.val().trim();
                        nameField.val(loadingHint).addClass(&quot; , &quot;'&quot; , &quot;ac_loading&quot; , &quot;'&quot; , &quot;);
                        $.ajax({
                               url: &quot;/index.php/pim/getEmployeeListAjax&quot;,
                               data: &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
                               dataType: &quot; , &quot;'&quot; , &quot;json&quot; , &quot;'&quot; , &quot;,
                               success: function(employeeList){

                                     nameField.autocomplete(employeeList, {

                                                formatItem: function(item) {
                                                    return $(&quot; , &quot;'&quot; , &quot;&lt;div/>&quot; , &quot;'&quot; , &quot;).text(item.name).html();
                                                },
                                                formatResult: function(item) {
                                                    return item.name
                                                }
                                                
                                                ,matchContains:true
                                            }).result(function(event, item) {
                                                idStoreField.val(item.id);
                                                idStoreField.data(&quot; , &quot;'&quot; , &quot;item.name&quot; , &quot;'&quot; , &quot;, item.name);
                                            }

                                        );
                                         nameField.removeClass(&quot; , &quot;'&quot; , &quot;ac_loading&quot; , &quot;'&quot; , &quot;); 
                                        
                                         if(value==&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
                                            nameField.val(typeHint).addClass(hintClass);
                                         } else {
                                            nameField.val(value).addClass();
                                         }
                                    }
                             });
                 }
                
            }); // End of $(document).ready

                 
        



Id
  

Employment Status
  
All
Freelance
Full-Time Contract
Full-Time Permanent
Full-Time Probation
Part-Time Contract
Part-Time Internship


Include
  
Current Employees Only
Current and Past Employees
Past Employees Only


Supervisor Name
  

Job Title
  
All
Automation Tester
BTest
Chief Executive Officer
Chief Financial Officer
Content Specialist
Customer Success Manager
Database Administrator
EA
Engineer
Finance Manager
Financial Analyst
Head of Support
HR Associate
HR Manager
IT Manager
Network Administrator
Payroll Administrator
Phó phòng đào tạo
Pre-Sales Coordinator
QA Engineer
QA Lead
Sales Representative
Social Media Marketer
Software Architect
Software Engineer
Support Specialist
VP - Client Services
VP - Sales &amp; Marketing


Sub Unit
  
All
Administration
Engineering
  Development
  Quality Assurance
  TechOps
Sales &amp; Marketing
  Sales
  Marketing
Client Services
  Technical Support
Finance
Human Resources



                

                
                                 

                
                    
                                        
                

            

        

     

    >

 


    
        
    
    

                
            
    
                
           
        

        


 1-50 of 62 First Previous 1 2 Next Last  

         

        


         
        

        
            
            
                
        
        
            
                                
                
Id
First (&amp; Middle) Name
Last Name
Job Title
Employment Status
Sub Unit
Supervisor
            
                            

            
                                            
                                                    0272
                                                    Admin
                                                    A
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0273
                                                    Admin
                                                    A
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0345
                                                    Odis
                                                    Adalwin
                                                    Engineer
                                                    Full-Time Permanent
                                                    Administration
                                                    John  Smith
                                            
                                            
                                                    0007
                                                    Peter Mac
                                                    Anderson
                                                    Chief Financial Officer
                                                    Full-Time Probation
                                                    Administration
                                                    John  Smith
                                            
                                            
                                                    0016
                                                    Linda Jane
                                                    Anderson
                                                    VP - Sales &amp; Marketing
                                                    Full-Time Permanent
                                                    Administration
                                                    John  Smith
                                            
                                            
                                                    0208
                                                    Lisa
                                                    Andrews
                                                    Software Engineer
                                                    Full-Time Probation
                                                    Development
                                                    Fiona  Grace
                                            
                                            
                                                    0247
                                                    Kallyani
                                                    Bhute
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0204
                                                    Cecil
                                                    Bonaparte
                                                    Software Engineer
                                                    Full-Time Permanent
                                                    Development
                                                    Fiona  Grace
                                            
                                            
                                                    0212
                                                    Charlie
                                                    Carter
                                                    QA Engineer
                                                    Full-Time Probation
                                                    Quality Assurance
                                                    Aaliyah  Haq
                                            
                                            
                                                    0020
                                                    Dominic
                                                    Chase
                                                    VP - Client Services
                                                    Full-Time Permanent
                                                    Administration
                                                    John  Smith
                                            
                                            
                                                    0217
                                                    Chenzira
                                                    Chuki
                                                    QA Engineer
                                                    Full-Time Permanent
                                                    Quality Assurance
                                                    Aaliyah  Haq
                                            
                                            
                                                    6117
                                                    Paul
                                                    Collings
                                                    HR Manager
                                                    Full-Time Permanent
                                                    Human Resources
                                                    John  Smith
                                            
                                            
                                                    0251
                                                    Ananya
                                                    Dash
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0253
                                                    Inba Venba
                                                    Dhiya
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0258
                                                    Inba Venba
                                                    Dhiya
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0250
                                                    Prajakta
                                                    Dhumal
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0221
                                                    Alice
                                                    Duval
                                                    Account Assistant (Deleted)
                                                    Full-Time Contract
                                                    Finance
                                                    Kevin  Mathews
                                            
                                            
                                                    0227
                                                    Ehioze
                                                    Ebo
                                                    Account Assistant (Deleted)
                                                    Full-Time Probation
                                                    Finance
                                                    Kevin  Mathews
                                            
                                            
                                                    0231
                                                    Nathan
                                                    Elliot
                                                    Sales Representative
                                                    Full-Time Probation
                                                    Sales
                                                    Linda  Jane  Anderson
                                            
                                            
                                                    0235
                                                    Goutam
                                                    Ganesh
                                                    Sales Representative
                                                    Full-Time Permanent
                                                    Sales
                                                    Linda  Jane  Anderson
                                            
                                            
                                                    0029
                                                    Fiona
                                                    Grace
                                                    Software Architect
                                                    Full-Time Permanent
                                                    Development
                                                    Odis  Adalwin
                                            
                                            
                                                    0034
                                                    Russel
                                                    Hamilton
                                                    Software Engineer
                                                    Full-Time Permanent
                                                    Development
                                                    Fiona  Grace
                                            
                                            
                                                    0038
                                                    Aaliyah
                                                    Haq
                                                    QA Lead
                                                    Full-Time Permanent
                                                    Quality Assurance
                                                    Odis  Adalwin
                                            
                                            
                                                    0042
                                                    Rebecca
                                                    Harmony
                                                    QA Engineer
                                                    Full-Time Contract
                                                    Quality Assurance
                                                    Aaliyah  Haq
                                            
                                            
                                                    0046
                                                    Cassidy
                                                    Hope
                                                    IT Manager
                                                    Part-Time Contract
                                                    TechOps
                                                    Odis  Adalwin
                                            
                                            
                                                    0239
                                                    Kiyara
                                                    Hu
                                                    HR Associate
                                                    Full-Time Permanent
                                                    Human Resources
                                                    Paul  Collings
                                            
                                            
                                                    0243
                                                    Jadine
                                                    Jackie
                                                    HR Associate
                                                    Full-Time Probation
                                                    Human Resources
                                                    Paul  Collings
                                            
                                            
                                                    0278
                                                    Harry
                                                    Kane
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    1289
                                                    Ali Mohd
                                                    Khan
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0248
                                                    Manali
                                                    Kulkarni
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0264
                                                    jyothi
                                                    lakshmisetty
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0262
                                                    sravani
                                                    mallikarjun
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0265
                                                    sravani
                                                    mallikarjun
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0267
                                                    sravani
                                                    mallikarjun
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0269
                                                    sravani
                                                    mallikarjun
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0271
                                                    sravani
                                                    mallikarjun
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0279
                                                    sravani
                                                    mallikarjun
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0050
                                                    Maggie
                                                    Manning
                                                    Network Administrator
                                                    Full-Time Probation
                                                    TechOps
                                                    Cassidy  Hope
                                            
                                            
                                                    0054
                                                    Jordan
                                                    Mathews
                                                    Database Administrator
                                                    Full-Time Contract
                                                    TechOps
                                                    Cassidy  Hope
                                            
                                            
                                                    0058
                                                    Kevin
                                                    Mathews
                                                    Finance Manager
                                                    Full-Time Permanent
                                                    Finance
                                                    Peter  Mac  Anderson
                                            
                                            
                                                    0275
                                                    bhakti sai
                                                    more
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0280
                                                    bhakti sai
                                                    more
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0062
                                                    Jasmine
                                                    Morgan
                                                    Financial Analyst
                                                    Full-Time Probation
                                                    Finance
                                                    Kevin  Mathews
                                            
                                            
                                                    0066
                                                    David
                                                    Morris
                                                    Account Assistant (Deleted)
                                                    Full-Time Permanent
                                                    Finance
                                                    Kevin  Mathews
                                            
                                            
                                                    0255
                                                    La Al
                                                    Na
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0070
                                                    Anthony
                                                    Nolan
                                                    Sales Representative
                                                    Full-Time Probation
                                                    Sales
                                                    Linda  Jane  Anderson
                                            
                                            
                                                    0074
                                                    Nina
                                                    Patel
                                                    Pre-Sales Coordinator
                                                    Full-Time Contract
                                                    Sales
                                                    Anthony  Nolan
                                            
                                            
                                                    0078
                                                    Melan
                                                    Peiris
                                                    Social Media Marketer
                                                    Full-Time Permanent
                                                    Marketing
                                                    Linda  Jane  Anderson
                                            
                                            
                                                    0260
                                                    Varmaa
                                                    Rajeshh
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0277
                                                    Varmaa
                                                    Rajeshh
                                                    
                                                    
                                                    
                                                    
                                            
                                
                            
         
                    
                                
                1-50 of 62 First Previous 1 2 Next Last            
                  
                
        
     
        
 




                    

                        var rootPath = &quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot;;

                        $(document).ready(function() {
                            ohrmList_init();

                        });
                                              





  
    ×
    OrangeHRM - Confirmation Required
  
  
    Delete records?
  
  
    
    
  




    $(document).ready(function() {
        
        var supervisors = [{&quot;name&quot;:&quot;Odis Adalwin&quot;,&quot;id&quot;:&quot;2&quot;},{&quot;name&quot;:&quot;Linda Jane Anderson&quot;,&quot;id&quot;:&quot;5&quot;},{&quot;name&quot;:&quot;Peter Mac Anderson&quot;,&quot;id&quot;:&quot;3&quot;},{&quot;name&quot;:&quot;Dominic Chase&quot;,&quot;id&quot;:&quot;6&quot;},{&quot;name&quot;:&quot;Paul Collings&quot;,&quot;id&quot;:&quot;7&quot;},{&quot;name&quot;:&quot;Fiona Grace&quot;,&quot;id&quot;:&quot;8&quot;},{&quot;name&quot;:&quot;Aaliyah Haq&quot;,&quot;id&quot;:&quot;10&quot;},{&quot;name&quot;:&quot;Cassidy Hope&quot;,&quot;id&quot;:&quot;12&quot;},{&quot;name&quot;:&quot;Kevin Mathews&quot;,&quot;id&quot;:&quot;15&quot;},{&quot;name&quot;:&quot;Anthony Nolan&quot;,&quot;id&quot;:&quot;18&quot;},{&quot;name&quot;:&quot;Melan Peiris&quot;,&quot;id&quot;:&quot;20&quot;},{&quot;name&quot;:&quot;DilshadS S&quot;,&quot;id&quot;:&quot;43&quot;},{&quot;name&quot;:&quot;John Smith&quot;,&quot;id&quot;:&quot;4&quot;}];

        $(&quot; , &quot;'&quot; , &quot;#btnDelete&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);

        $(&quot;#ohrmList_chkSelectAll&quot;).click(function() {
            if ($(&quot;:checkbox&quot;).length == 1) {
                $(&quot; , &quot;'&quot; , &quot;#btnDelete&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
            }
            else {
                if ($(&quot;#ohrmList_chkSelectAll&quot;).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;)) {
                    $(&quot; , &quot;'&quot; , &quot;#btnDelete&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
                } else {
                    $(&quot; , &quot;'&quot; , &quot;#btnDelete&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
                }
            }
        });

        $(&quot; , &quot;'&quot; , &quot;:checkbox[name*=&quot;chkSelectRow[]&quot;]&quot; , &quot;'&quot; , &quot;).click(function() {
            if ($(&quot; , &quot;'&quot; , &quot;:checkbox[name*=&quot;chkSelectRow[]&quot;]&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;)) {
                $(&quot; , &quot;'&quot; , &quot;#btnDelete&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
            } else {
                $(&quot; , &quot;'&quot; , &quot;#btnDelete&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
            }
        });

        // Handle hints
        if ($(&quot;#empsearch_id&quot;).val() == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
            $(&quot;#empsearch_id&quot;).val(&quot; , &quot;'&quot; , &quot;Type Employee Id...&quot; , &quot;'&quot; , &quot;)
                    .addClass(&quot;inputFormatHint&quot;);
        }

        if ($(&quot;#empsearch_supervisor_name&quot;).val() == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
            $(&quot;#empsearch_supervisor_name&quot;).val(&quot; , &quot;'&quot; , &quot;Type for hints...&quot; , &quot;'&quot; , &quot;)
                    .addClass(&quot;inputFormatHint&quot;);
        }

        $(&quot;#empsearch_id, #empsearch_supervisor_name&quot;).one(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, function() {

            if ($(this).hasClass(&quot;inputFormatHint&quot;)) {
                $(this).val(&quot;&quot;);
                $(this).removeClass(&quot;inputFormatHint&quot;);
            }
        });

        $(&quot;#empsearch_supervisor_name&quot;).autocomplete(supervisors, {
            formatItem: function(item) {
                return $(&quot; , &quot;'&quot; , &quot;&lt;div/>&quot; , &quot;'&quot; , &quot;).text(item.name).html();
            },
            formatResult: function(item) {
                return item.name
            }
            , matchContains: true
        }).result(function(event, item) {
        }
        );

        $(&quot; , &quot;'&quot; , &quot;#searchBtn&quot; , &quot;'&quot; , &quot;).click(function() {
            $(&quot;#empsearch_isSubmitted&quot;).val(&quot; , &quot;'&quot; , &quot;yes&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;#search_form input.inputFormatHint&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;#search_form input.ac_loading&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;#search_form&quot; , &quot;'&quot; , &quot;).submit();
        });

        $(&quot; , &quot;'&quot; , &quot;#resetBtn&quot; , &quot;'&quot; , &quot;).click(function() {
            $(&quot;#empsearch_isSubmitted&quot;).val(&quot; , &quot;'&quot; , &quot;yes&quot; , &quot;'&quot; , &quot;);
            $(&quot;#empsearch_employee_name_empName&quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            $(&quot;#empsearch_supervisor_name&quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            $(&quot;#empsearch_id&quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            $(&quot;#empsearch_job_title&quot;).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
            $(&quot;#empsearch_employee_status&quot;).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
            $(&quot;#empsearch_sub_unit&quot;).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
            $(&quot;#empsearch_termination&quot;).val(&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;#search_form&quot; , &quot;'&quot; , &quot;).submit();
        });

        $(&quot; , &quot;'&quot; , &quot;#btnAdd&quot; , &quot;'&quot; , &quot;).click(function() {
            location.href = &quot;/index.php/pim/addEmployee&quot;;
        });
        $(&quot; , &quot;'&quot; , &quot;#btnDelete&quot; , &quot;'&quot; , &quot;).click(function() {
            $(&quot; , &quot;'&quot; , &quot;#frmList_ohrmListComponent&quot; , &quot;'&quot; , &quot;).submit(function() {
                $(&quot; , &quot;'&quot; , &quot;#deleteConfirmation&quot; , &quot;'&quot; , &quot;).dialog(&quot; , &quot;'&quot; , &quot;open&quot; , &quot;'&quot; , &quot;);
                return false;
            });
        });

        /* Delete confirmation controls: Begin */
        $(&quot; , &quot;'&quot; , &quot;#dialogDeleteBtn&quot; , &quot;'&quot; , &quot;).click(function() {
            document.frmList_ohrmListComponent.submit();
        });
        /* Delete confirmation controls: End */

    }); //ready

    function submitPage(pageNo) {
        document.frmEmployeeSearch.pageNo.value = pageNo;
        document.frmEmployeeSearch.hdnAction.value = &quot; , &quot;'&quot; , &quot;paging&quot; , &quot;'&quot; , &quot;;
        $(&quot; , &quot;'&quot; , &quot;#search_form input.inputFormatHint&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;#search_form input.ac_loading&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
        $(&quot;#empsearch_isSubmitted&quot;).val(&quot; , &quot;'&quot; , &quot;no&quot; , &quot;'&quot; , &quot;);
        document.getElementById(&quot; , &quot;'&quot; , &quot;search_form&quot; , &quot;'&quot; , &quot;).submit();
    }


            &quot;) or . = concat(&quot;

                  

    
        Employee Information
    
    
        



        

            

                
                    Employee Name
  





        

            var employees_empsearch_employee_name = [];

            $(document).ready(function() {
            
                var nameField = $(&quot;#empsearch_employee_name_empName&quot;);
                var idStoreField = $(&quot;#empsearch_employee_name_empId&quot;);
                var typeHint = &quot; , &quot;'&quot; , &quot;Type for hints...&quot; , &quot;'&quot; , &quot;;
                var hintClass = &quot; , &quot;'&quot; , &quot;inputFormatHint&quot; , &quot;'&quot; , &quot;;
                var loadingMethod = &quot; , &quot;'&quot; , &quot;ajax&quot; , &quot;'&quot; , &quot;;
                var loadingHint = &quot; , &quot;'&quot; , &quot;Loading&quot; , &quot;'&quot; , &quot;;
            
                if (idStoreField.val() != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
                    idStoreField.data(&quot; , &quot;'&quot; , &quot;item.name&quot; , &quot;'&quot; , &quot;, nameField.val());
                }
                
                nameField.data(&quot; , &quot;'&quot; , &quot;typeHint&quot; , &quot;'&quot; , &quot;, typeHint);
                nameField.data(&quot; , &quot;'&quot; , &quot;loadingHint&quot; , &quot;'&quot; , &quot;, loadingHint);
                
                nameField.one(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, function() {

                        if ($(this).hasClass(hintClass)) {
                            $(this).val(&quot;&quot;);
                            $(this).removeClass(hintClass);
                        }

                    });
                    
                if( loadingMethod != &quot; , &quot;'&quot; , &quot;ajax&quot; , &quot;'&quot; , &quot;){
                    if (nameField.val() == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; || nameField.val() == typeHint) {
                        nameField.val(typeHint).addClass(hintClass);
                    }

                    

                    nameField.autocomplete(employees_empsearch_employee_name, {

                        formatItem: function(item) {
                            return $(&quot; , &quot;'&quot; , &quot;&lt;div/>&quot; , &quot;'&quot; , &quot;).text(item.name).html();
                        },
                        formatResult: function(item) {
                            return item.name
                        }
                      ,matchContains:true
                        }).result(function(event, item) {
                            idStoreField.val(item.id);
                            idStoreField.data(&quot; , &quot;'&quot; , &quot;item.name&quot; , &quot;'&quot; , &quot;, item.name);
                        }

                    );
                 }else{
                        var value = nameField.val().trim();
                        nameField.val(loadingHint).addClass(&quot; , &quot;'&quot; , &quot;ac_loading&quot; , &quot;'&quot; , &quot;);
                        $.ajax({
                               url: &quot;/index.php/pim/getEmployeeListAjax&quot;,
                               data: &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
                               dataType: &quot; , &quot;'&quot; , &quot;json&quot; , &quot;'&quot; , &quot;,
                               success: function(employeeList){

                                     nameField.autocomplete(employeeList, {

                                                formatItem: function(item) {
                                                    return $(&quot; , &quot;'&quot; , &quot;&lt;div/>&quot; , &quot;'&quot; , &quot;).text(item.name).html();
                                                },
                                                formatResult: function(item) {
                                                    return item.name
                                                }
                                                
                                                ,matchContains:true
                                            }).result(function(event, item) {
                                                idStoreField.val(item.id);
                                                idStoreField.data(&quot; , &quot;'&quot; , &quot;item.name&quot; , &quot;'&quot; , &quot;, item.name);
                                            }

                                        );
                                         nameField.removeClass(&quot; , &quot;'&quot; , &quot;ac_loading&quot; , &quot;'&quot; , &quot;); 
                                        
                                         if(value==&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
                                            nameField.val(typeHint).addClass(hintClass);
                                         } else {
                                            nameField.val(value).addClass();
                                         }
                                    }
                             });
                 }
                
            }); // End of $(document).ready

                 
        



Id
  

Employment Status
  
All
Freelance
Full-Time Contract
Full-Time Permanent
Full-Time Probation
Part-Time Contract
Part-Time Internship


Include
  
Current Employees Only
Current and Past Employees
Past Employees Only


Supervisor Name
  

Job Title
  
All
Automation Tester
BTest
Chief Executive Officer
Chief Financial Officer
Content Specialist
Customer Success Manager
Database Administrator
EA
Engineer
Finance Manager
Financial Analyst
Head of Support
HR Associate
HR Manager
IT Manager
Network Administrator
Payroll Administrator
Phó phòng đào tạo
Pre-Sales Coordinator
QA Engineer
QA Lead
Sales Representative
Social Media Marketer
Software Architect
Software Engineer
Support Specialist
VP - Client Services
VP - Sales &amp; Marketing


Sub Unit
  
All
Administration
Engineering
  Development
  Quality Assurance
  TechOps
Sales &amp; Marketing
  Sales
  Marketing
Client Services
  Technical Support
Finance
Human Resources



                

                
                                 

                
                    
                                        
                

            

        

     

    >

 


    
        
    
    

                
            
    
                
           
        

        


 1-50 of 62 First Previous 1 2 Next Last  

         

        


         
        

        
            
            
                
        
        
            
                                
                
Id
First (&amp; Middle) Name
Last Name
Job Title
Employment Status
Sub Unit
Supervisor
            
                            

            
                                            
                                                    0272
                                                    Admin
                                                    A
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0273
                                                    Admin
                                                    A
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0345
                                                    Odis
                                                    Adalwin
                                                    Engineer
                                                    Full-Time Permanent
                                                    Administration
                                                    John  Smith
                                            
                                            
                                                    0007
                                                    Peter Mac
                                                    Anderson
                                                    Chief Financial Officer
                                                    Full-Time Probation
                                                    Administration
                                                    John  Smith
                                            
                                            
                                                    0016
                                                    Linda Jane
                                                    Anderson
                                                    VP - Sales &amp; Marketing
                                                    Full-Time Permanent
                                                    Administration
                                                    John  Smith
                                            
                                            
                                                    0208
                                                    Lisa
                                                    Andrews
                                                    Software Engineer
                                                    Full-Time Probation
                                                    Development
                                                    Fiona  Grace
                                            
                                            
                                                    0247
                                                    Kallyani
                                                    Bhute
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0204
                                                    Cecil
                                                    Bonaparte
                                                    Software Engineer
                                                    Full-Time Permanent
                                                    Development
                                                    Fiona  Grace
                                            
                                            
                                                    0212
                                                    Charlie
                                                    Carter
                                                    QA Engineer
                                                    Full-Time Probation
                                                    Quality Assurance
                                                    Aaliyah  Haq
                                            
                                            
                                                    0020
                                                    Dominic
                                                    Chase
                                                    VP - Client Services
                                                    Full-Time Permanent
                                                    Administration
                                                    John  Smith
                                            
                                            
                                                    0217
                                                    Chenzira
                                                    Chuki
                                                    QA Engineer
                                                    Full-Time Permanent
                                                    Quality Assurance
                                                    Aaliyah  Haq
                                            
                                            
                                                    6117
                                                    Paul
                                                    Collings
                                                    HR Manager
                                                    Full-Time Permanent
                                                    Human Resources
                                                    John  Smith
                                            
                                            
                                                    0251
                                                    Ananya
                                                    Dash
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0253
                                                    Inba Venba
                                                    Dhiya
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0258
                                                    Inba Venba
                                                    Dhiya
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0250
                                                    Prajakta
                                                    Dhumal
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0221
                                                    Alice
                                                    Duval
                                                    Account Assistant (Deleted)
                                                    Full-Time Contract
                                                    Finance
                                                    Kevin  Mathews
                                            
                                            
                                                    0227
                                                    Ehioze
                                                    Ebo
                                                    Account Assistant (Deleted)
                                                    Full-Time Probation
                                                    Finance
                                                    Kevin  Mathews
                                            
                                            
                                                    0231
                                                    Nathan
                                                    Elliot
                                                    Sales Representative
                                                    Full-Time Probation
                                                    Sales
                                                    Linda  Jane  Anderson
                                            
                                            
                                                    0235
                                                    Goutam
                                                    Ganesh
                                                    Sales Representative
                                                    Full-Time Permanent
                                                    Sales
                                                    Linda  Jane  Anderson
                                            
                                            
                                                    0029
                                                    Fiona
                                                    Grace
                                                    Software Architect
                                                    Full-Time Permanent
                                                    Development
                                                    Odis  Adalwin
                                            
                                            
                                                    0034
                                                    Russel
                                                    Hamilton
                                                    Software Engineer
                                                    Full-Time Permanent
                                                    Development
                                                    Fiona  Grace
                                            
                                            
                                                    0038
                                                    Aaliyah
                                                    Haq
                                                    QA Lead
                                                    Full-Time Permanent
                                                    Quality Assurance
                                                    Odis  Adalwin
                                            
                                            
                                                    0042
                                                    Rebecca
                                                    Harmony
                                                    QA Engineer
                                                    Full-Time Contract
                                                    Quality Assurance
                                                    Aaliyah  Haq
                                            
                                            
                                                    0046
                                                    Cassidy
                                                    Hope
                                                    IT Manager
                                                    Part-Time Contract
                                                    TechOps
                                                    Odis  Adalwin
                                            
                                            
                                                    0239
                                                    Kiyara
                                                    Hu
                                                    HR Associate
                                                    Full-Time Permanent
                                                    Human Resources
                                                    Paul  Collings
                                            
                                            
                                                    0243
                                                    Jadine
                                                    Jackie
                                                    HR Associate
                                                    Full-Time Probation
                                                    Human Resources
                                                    Paul  Collings
                                            
                                            
                                                    0278
                                                    Harry
                                                    Kane
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    1289
                                                    Ali Mohd
                                                    Khan
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0248
                                                    Manali
                                                    Kulkarni
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0264
                                                    jyothi
                                                    lakshmisetty
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0262
                                                    sravani
                                                    mallikarjun
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0265
                                                    sravani
                                                    mallikarjun
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0267
                                                    sravani
                                                    mallikarjun
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0269
                                                    sravani
                                                    mallikarjun
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0271
                                                    sravani
                                                    mallikarjun
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0279
                                                    sravani
                                                    mallikarjun
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0050
                                                    Maggie
                                                    Manning
                                                    Network Administrator
                                                    Full-Time Probation
                                                    TechOps
                                                    Cassidy  Hope
                                            
                                            
                                                    0054
                                                    Jordan
                                                    Mathews
                                                    Database Administrator
                                                    Full-Time Contract
                                                    TechOps
                                                    Cassidy  Hope
                                            
                                            
                                                    0058
                                                    Kevin
                                                    Mathews
                                                    Finance Manager
                                                    Full-Time Permanent
                                                    Finance
                                                    Peter  Mac  Anderson
                                            
                                            
                                                    0275
                                                    bhakti sai
                                                    more
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0280
                                                    bhakti sai
                                                    more
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0062
                                                    Jasmine
                                                    Morgan
                                                    Financial Analyst
                                                    Full-Time Probation
                                                    Finance
                                                    Kevin  Mathews
                                            
                                            
                                                    0066
                                                    David
                                                    Morris
                                                    Account Assistant (Deleted)
                                                    Full-Time Permanent
                                                    Finance
                                                    Kevin  Mathews
                                            
                                            
                                                    0255
                                                    La Al
                                                    Na
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0070
                                                    Anthony
                                                    Nolan
                                                    Sales Representative
                                                    Full-Time Probation
                                                    Sales
                                                    Linda  Jane  Anderson
                                            
                                            
                                                    0074
                                                    Nina
                                                    Patel
                                                    Pre-Sales Coordinator
                                                    Full-Time Contract
                                                    Sales
                                                    Anthony  Nolan
                                            
                                            
                                                    0078
                                                    Melan
                                                    Peiris
                                                    Social Media Marketer
                                                    Full-Time Permanent
                                                    Marketing
                                                    Linda  Jane  Anderson
                                            
                                            
                                                    0260
                                                    Varmaa
                                                    Rajeshh
                                                    
                                                    
                                                    
                                                    
                                            
                                            
                                                    0277
                                                    Varmaa
                                                    Rajeshh
                                                    
                                                    
                                                    
                                                    
                                            
                                
                            
         
                    
                                
                1-50 of 62 First Previous 1 2 Next Last            
                  
                
        
     
        
 




                    

                        var rootPath = &quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot;;

                        $(document).ready(function() {
                            ohrmList_init();

                        });
                                              





  
    ×
    OrangeHRM - Confirmation Required
  
  
    Delete records?
  
  
    
    
  




    $(document).ready(function() {
        
        var supervisors = [{&quot;name&quot;:&quot;Odis Adalwin&quot;,&quot;id&quot;:&quot;2&quot;},{&quot;name&quot;:&quot;Linda Jane Anderson&quot;,&quot;id&quot;:&quot;5&quot;},{&quot;name&quot;:&quot;Peter Mac Anderson&quot;,&quot;id&quot;:&quot;3&quot;},{&quot;name&quot;:&quot;Dominic Chase&quot;,&quot;id&quot;:&quot;6&quot;},{&quot;name&quot;:&quot;Paul Collings&quot;,&quot;id&quot;:&quot;7&quot;},{&quot;name&quot;:&quot;Fiona Grace&quot;,&quot;id&quot;:&quot;8&quot;},{&quot;name&quot;:&quot;Aaliyah Haq&quot;,&quot;id&quot;:&quot;10&quot;},{&quot;name&quot;:&quot;Cassidy Hope&quot;,&quot;id&quot;:&quot;12&quot;},{&quot;name&quot;:&quot;Kevin Mathews&quot;,&quot;id&quot;:&quot;15&quot;},{&quot;name&quot;:&quot;Anthony Nolan&quot;,&quot;id&quot;:&quot;18&quot;},{&quot;name&quot;:&quot;Melan Peiris&quot;,&quot;id&quot;:&quot;20&quot;},{&quot;name&quot;:&quot;DilshadS S&quot;,&quot;id&quot;:&quot;43&quot;},{&quot;name&quot;:&quot;John Smith&quot;,&quot;id&quot;:&quot;4&quot;}];

        $(&quot; , &quot;'&quot; , &quot;#btnDelete&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);

        $(&quot;#ohrmList_chkSelectAll&quot;).click(function() {
            if ($(&quot;:checkbox&quot;).length == 1) {
                $(&quot; , &quot;'&quot; , &quot;#btnDelete&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
            }
            else {
                if ($(&quot;#ohrmList_chkSelectAll&quot;).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;)) {
                    $(&quot; , &quot;'&quot; , &quot;#btnDelete&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
                } else {
                    $(&quot; , &quot;'&quot; , &quot;#btnDelete&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
                }
            }
        });

        $(&quot; , &quot;'&quot; , &quot;:checkbox[name*=&quot;chkSelectRow[]&quot;]&quot; , &quot;'&quot; , &quot;).click(function() {
            if ($(&quot; , &quot;'&quot; , &quot;:checkbox[name*=&quot;chkSelectRow[]&quot;]&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;)) {
                $(&quot; , &quot;'&quot; , &quot;#btnDelete&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
            } else {
                $(&quot; , &quot;'&quot; , &quot;#btnDelete&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
            }
        });

        // Handle hints
        if ($(&quot;#empsearch_id&quot;).val() == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
            $(&quot;#empsearch_id&quot;).val(&quot; , &quot;'&quot; , &quot;Type Employee Id...&quot; , &quot;'&quot; , &quot;)
                    .addClass(&quot;inputFormatHint&quot;);
        }

        if ($(&quot;#empsearch_supervisor_name&quot;).val() == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
            $(&quot;#empsearch_supervisor_name&quot;).val(&quot; , &quot;'&quot; , &quot;Type for hints...&quot; , &quot;'&quot; , &quot;)
                    .addClass(&quot;inputFormatHint&quot;);
        }

        $(&quot;#empsearch_id, #empsearch_supervisor_name&quot;).one(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, function() {

            if ($(this).hasClass(&quot;inputFormatHint&quot;)) {
                $(this).val(&quot;&quot;);
                $(this).removeClass(&quot;inputFormatHint&quot;);
            }
        });

        $(&quot;#empsearch_supervisor_name&quot;).autocomplete(supervisors, {
            formatItem: function(item) {
                return $(&quot; , &quot;'&quot; , &quot;&lt;div/>&quot; , &quot;'&quot; , &quot;).text(item.name).html();
            },
            formatResult: function(item) {
                return item.name
            }
            , matchContains: true
        }).result(function(event, item) {
        }
        );

        $(&quot; , &quot;'&quot; , &quot;#searchBtn&quot; , &quot;'&quot; , &quot;).click(function() {
            $(&quot;#empsearch_isSubmitted&quot;).val(&quot; , &quot;'&quot; , &quot;yes&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;#search_form input.inputFormatHint&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;#search_form input.ac_loading&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;#search_form&quot; , &quot;'&quot; , &quot;).submit();
        });

        $(&quot; , &quot;'&quot; , &quot;#resetBtn&quot; , &quot;'&quot; , &quot;).click(function() {
            $(&quot;#empsearch_isSubmitted&quot;).val(&quot; , &quot;'&quot; , &quot;yes&quot; , &quot;'&quot; , &quot;);
            $(&quot;#empsearch_employee_name_empName&quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            $(&quot;#empsearch_supervisor_name&quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            $(&quot;#empsearch_id&quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            $(&quot;#empsearch_job_title&quot;).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
            $(&quot;#empsearch_employee_status&quot;).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
            $(&quot;#empsearch_sub_unit&quot;).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
            $(&quot;#empsearch_termination&quot;).val(&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;#search_form&quot; , &quot;'&quot; , &quot;).submit();
        });

        $(&quot; , &quot;'&quot; , &quot;#btnAdd&quot; , &quot;'&quot; , &quot;).click(function() {
            location.href = &quot;/index.php/pim/addEmployee&quot;;
        });
        $(&quot; , &quot;'&quot; , &quot;#btnDelete&quot; , &quot;'&quot; , &quot;).click(function() {
            $(&quot; , &quot;'&quot; , &quot;#frmList_ohrmListComponent&quot; , &quot;'&quot; , &quot;).submit(function() {
                $(&quot; , &quot;'&quot; , &quot;#deleteConfirmation&quot; , &quot;'&quot; , &quot;).dialog(&quot; , &quot;'&quot; , &quot;open&quot; , &quot;'&quot; , &quot;);
                return false;
            });
        });

        /* Delete confirmation controls: Begin */
        $(&quot; , &quot;'&quot; , &quot;#dialogDeleteBtn&quot; , &quot;'&quot; , &quot;).click(function() {
            document.frmList_ohrmListComponent.submit();
        });
        /* Delete confirmation controls: End */

    }); //ready

    function submitPage(pageNo) {
        document.frmEmployeeSearch.pageNo.value = pageNo;
        document.frmEmployeeSearch.hdnAction.value = &quot; , &quot;'&quot; , &quot;paging&quot; , &quot;'&quot; , &quot;;
        $(&quot; , &quot;'&quot; , &quot;#search_form input.inputFormatHint&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;#search_form input.ac_loading&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
        $(&quot;#empsearch_isSubmitted&quot;).val(&quot; , &quot;'&quot; , &quot;no&quot; , &quot;'&quot; , &quot;);
        document.getElementById(&quot; , &quot;'&quot; , &quot;search_form&quot; , &quot;'&quot; , &quot;).submit();
    }


            &quot;))]</value>
      <webElementGuid>0f474bb6-d59d-49df-a2a0-9fcca245689b</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
