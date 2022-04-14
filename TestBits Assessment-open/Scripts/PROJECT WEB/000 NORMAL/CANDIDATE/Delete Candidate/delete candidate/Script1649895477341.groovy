import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.callTestCase(findTestCase('PROJECT WEB/000 NORMAL/login/login'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/PROJECT WEB/Page_OrangeHRM/add cadidate/Page_OrangeHRM/b_Recruitment'))

WebUI.setText(findTestObject('Object Repository/PROJECT WEB/candidate/Page_OrangeHRM/input_Candidate Name_candidateSearchcandidateName'), 
    'John Wills Junior')

WebUI.click(findTestObject('Object Repository/PROJECT WEB/candidate/Page_OrangeHRM/input_Method of Application_btnSrch'))

WebUI.click(findTestObject('Object Repository/PROJECT WEB/candidate/Page_OrangeHRM/input_Resume_chkSelectRow'))

WebUI.click(findTestObject('Object Repository/PROJECT WEB/candidate/Page_OrangeHRM/input_Method of Application_btnDelete'))

WebUI.click(findTestObject('Object Repository/PROJECT WEB/candidate/Page_OrangeHRM/input_OrangeHRM - Confirmation Required_dia_9fc7db'))

WebUI.click(findTestObject('PROJECT WEB/candidate/div_Successfully Deleted       Close'))

