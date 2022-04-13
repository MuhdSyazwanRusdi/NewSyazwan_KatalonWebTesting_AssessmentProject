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

WebUI.click(findTestObject('Object Repository/PROJECT WEB/employee/b_PIM'))

WebUI.click(findTestObject('Object Repository/PROJECT WEB/employee/input_Sub Unit_btnAdd'))

WebUI.setText(findTestObject('Object Repository/PROJECT WEB/employee/input__firstName'), 'Ali')

WebUI.setText(findTestObject('Object Repository/PROJECT WEB/employee/input_Middle Name_middleName'), 'Mohd')

WebUI.setText(findTestObject('Object Repository/PROJECT WEB/employee/input__lastName'), 'Khan')

WebUI.setText(findTestObject('PROJECT WEB/employee/input_Employee Id_employeeId'), '0289')

WebUI.uploadFile(findTestObject('PROJECT WEB/employee/input_Photograph_photofile'), 'C:\\Users\\syazw\\Documents\\GitHub\\cat small.jpg')

WebUI.click(findTestObject('Object Repository/PROJECT WEB/employee/input_Create Login Details_chkLogin'))

WebUI.setText(findTestObject('Object Repository/PROJECT WEB/employee/input__user_name'), 'ahmadmadkhan')

WebUI.setText(findTestObject('Object Repository/PROJECT WEB/employee/input__user_password'), 'Tangansemut#25')

WebUI.setText(findTestObject('Object Repository/PROJECT WEB/employee/input__re_password'), 'sasasasasa')

WebUI.click(findTestObject('PROJECT WEB/employee/label_Status'))

WebUI.click(findTestObject('PROJECT WEB/employee/span_Passwords do not match'))

WebUI.closeBrowser()

