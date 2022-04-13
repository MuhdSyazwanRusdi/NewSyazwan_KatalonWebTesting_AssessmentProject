<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>ol_Full Name First Name Middle Name  Last N_a163be</name>
   <tag></tag>
   <elementGuidId>1b3c5f3d-0fcf-4d73-8091-ff89bc3603ea</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//form[@id='frmAddEmp']/fieldset/ol</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>#frmAddEmp > fieldset > ol</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>ol</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
                    Full Name* First Name
 

Middle Name
 

* Last Name
 


Employee Id
  

Photograph
  Accepts jpg, .png, .gif up to 1MB. Recommended dimensions: 200px X 200px

Create Login Details
  

User Name *
 

Password *
 Should have at least 8 charactersStrongestFor a strong password, please use a hard to guess combination of text with upper and lower case characters, symbols and numbers
        var passwordOptions = [
        'Very Weak', 'Weak', 'Better',
        'Medium', 'Strong', 'Strongest'
    ];
        
        $(document).ready(function() {
           $('#user_password').on('keyup', function(){
              showPasswordStrength($('#user_password').val(), '/index.php/securityAuthentication/getPasswordStrengthAjax', 'user_password', passwordOptions );
           });         
        });
    

Confirm Password *
 

Status *
 
Enabled
Disabled



                    
                        * Required field                    
                </value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;frmAddEmp&quot;)/fieldset[1]/ol[1]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//form[@id='frmAddEmp']/fieldset/ol</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Add Employee'])[2]/following::ol[1]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Buzz'])[1]/following::ol[1]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[3]/div/div[2]/form/fieldset/ol</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//ol[(text() = concat(&quot;
                    Full Name* First Name
 

Middle Name
 

* Last Name
 


Employee Id
  

Photograph
  Accepts jpg, .png, .gif up to 1MB. Recommended dimensions: 200px X 200px

Create Login Details
  

User Name *
 

Password *
 Should have at least 8 charactersStrongestFor a strong password, please use a hard to guess combination of text with upper and lower case characters, symbols and numbers
        var passwordOptions = [
        &quot; , &quot;'&quot; , &quot;Very Weak&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Weak&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Better&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;Medium&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Strong&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Strongest&quot; , &quot;'&quot; , &quot;
    ];
        
        $(document).ready(function() {
           $(&quot; , &quot;'&quot; , &quot;#user_password&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;keyup&quot; , &quot;'&quot; , &quot;, function(){
              showPasswordStrength($(&quot; , &quot;'&quot; , &quot;#user_password&quot; , &quot;'&quot; , &quot;).val(), &quot; , &quot;'&quot; , &quot;/index.php/securityAuthentication/getPasswordStrengthAjax&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;user_password&quot; , &quot;'&quot; , &quot;, passwordOptions );
           });         
        });
    

Confirm Password *
 

Status *
 
Enabled
Disabled



                    
                        * Required field                    
                &quot;) or . = concat(&quot;
                    Full Name* First Name
 

Middle Name
 

* Last Name
 


Employee Id
  

Photograph
  Accepts jpg, .png, .gif up to 1MB. Recommended dimensions: 200px X 200px

Create Login Details
  

User Name *
 

Password *
 Should have at least 8 charactersStrongestFor a strong password, please use a hard to guess combination of text with upper and lower case characters, symbols and numbers
        var passwordOptions = [
        &quot; , &quot;'&quot; , &quot;Very Weak&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Weak&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Better&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;Medium&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Strong&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Strongest&quot; , &quot;'&quot; , &quot;
    ];
        
        $(document).ready(function() {
           $(&quot; , &quot;'&quot; , &quot;#user_password&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;keyup&quot; , &quot;'&quot; , &quot;, function(){
              showPasswordStrength($(&quot; , &quot;'&quot; , &quot;#user_password&quot; , &quot;'&quot; , &quot;).val(), &quot; , &quot;'&quot; , &quot;/index.php/securityAuthentication/getPasswordStrengthAjax&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;user_password&quot; , &quot;'&quot; , &quot;, passwordOptions );
           });         
        });
    

Confirm Password *
 

Status *
 
Enabled
Disabled



                    
                        * Required field                    
                &quot;))]</value>
   </webElementXpaths>
</WebElementEntity>
