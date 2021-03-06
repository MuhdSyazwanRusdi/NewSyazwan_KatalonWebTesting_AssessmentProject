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

WebUI.rightClick(findTestObject('Object Repository/PROJECT WEB/vacai/Page_OrangeHRM/b_Recruitment'))

WebUI.click(findTestObject('Object Repository/PROJECT WEB/vacai/Page_OrangeHRM/a_Vacancies'))

WebUI.click(findTestObject('PROJECT WEB/vacai/delete/a_Apps Developer'))

WebUI.click(findTestObject('PROJECT WEB/vacai/delete/a_Apps Tester'))

WebUI.click(findTestObject('Object Repository/PROJECT WEB/vacai/Page_OrangeHRM/input_Status_btnDelete'))

WebUI.click(findTestObject('Object Repository/PROJECT WEB/vacai/Page_OrangeHRM/input_OrangeHRM - Confirmation Required_dia_9fc7db'))

WebUI.click(findTestObject('Object Repository/PROJECT WEB/vacai/Page_OrangeHRM/div_Successfully Deleted       Close'))

