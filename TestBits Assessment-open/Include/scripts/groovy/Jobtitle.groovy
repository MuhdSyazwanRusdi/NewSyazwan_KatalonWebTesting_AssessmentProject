import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testcase.TestCaseFactory
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testdata.TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import internal.GlobalVariable

import org.openqa.selenium.WebElement
import org.openqa.selenium.WebDriver
import org.openqa.selenium.By

import com.kms.katalon.core.mobile.keyword.internal.MobileDriverFactory
import com.kms.katalon.core.webui.driver.DriverFactory

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.testobject.ConditionType
import com.kms.katalon.core.testobject.TestObjectProperty

import com.kms.katalon.core.mobile.helper.MobileElementCommonHelper
import com.kms.katalon.core.util.KeywordUtil

import com.kms.katalon.core.webui.exception.WebElementNotFoundException

import cucumber.api.java.en.And
import cucumber.api.java.en.Given
import cucumber.api.java.en.Then
import cucumber.api.java.en.When



class Jobtitle {
	@Given("Login until job title")
	def loginjob(){

		WebUI.openBrowser('')

		WebUI.navigateToUrl('https://opensource-demo.orangehrmlive.com/')

		WebUI.setText(findTestObject('Object Repository/PROJECT WEB/Page_OrangeHRM/input_LOGIN Panel_txtUsername'), 'Admin')

		WebUI.setText(findTestObject('Object Repository/PROJECT WEB/Page_OrangeHRM/input_Username_txtPassword'), 'admin123')

		WebUI.click(findTestObject('Object Repository/PROJECT WEB/Page_OrangeHRM/input_Password_Submit'))

		WebUI.click(findTestObject('PROJECT WEB/job title/b_Admin'))

		WebUI.click(findTestObject('PROJECT WEB/job title/a_Job'))

		WebUI.click(findTestObject('PROJECT WEB/job title/a_Job Titles'))

		WebUI.click(findTestObject('PROJECT WEB/job title/input_Job Titles_btnAdd'))
	}


	@When("User enter (.*) and (.*)")
	def enter_job_description(String namee, String descp){

		WebUI.setText(findTestObject('PROJECT WEB/job title/input__jobTitlejobTitle'), namee)
		WebUI.setText(findTestObject('PROJECT WEB/job title/textarea_Job Description_jobTitlejobDescription'), descp)
		WebUI.setText(findTestObject('PROJECT WEB/job title/textarea_Note_jobTitlenote'), 'entahlah')
	}

	@And("User click save")
	def clicksace() {
		WebUI.click(findTestObject('PROJECT WEB/job title/input__btnSave'))
	}

	@And("User add file and click save")
	def add_click() {
		WebUI.uploadFile(findTestObject('PROJECT WEB/job title/input_Job Specification_jobTitlejobSpec'), 'C:\\Users\\syazw\\Documents\\GitHub\\resume big.pdf')
		WebUI.click(findTestObject('PROJECT WEB/job title/input__btnSave'))
	}
	
	
	@Then("User is navigated to jobtitle")
	def jobtitle_home() {
		WebUI.click(findTestObject('PROJECT WEB/job title/div_Successfully Saved       Close'))
	}
	
	@Then("Jobtitle is required")
	def jobtitle_required() {
		WebUI.click(findTestObject('PROJECT WEB/job title/span_Required'))
	}

	@Then("File exceed 1mb")
	def fileexceed() {
		WebUI.click(findTestObject('PROJECT WEB/job title/div_Validation Failed       Close'))
	}

	@Then("Jobtitle is used before")
	def jobtitle_same() {
		WebUI.click(findTestObject('PROJECT WEB/job title/span_Already exists'))
	}
	
	
	
}