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

WebUI.callTestCase(findTestCase('PROJECT WEB/000 NORMAL/LOCATION/EDIT LOCATION/sucess replace'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('PROJECT WEB/location/wrong format in edit/a_City Plaza wow'))

WebUI.click(findTestObject('PROJECT WEB/location/wrong format in edit/input__btnSave'))

WebUI.setText(findTestObject('PROJECT WEB/location/wrong format in edit/input_Phone_locationphone'), 'ewewewew')

WebUI.click(findTestObject('PROJECT WEB/location/wrong format in edit/ol_Name   Country   -- Select --Afghanistan_502f77'))

WebUI.setText(findTestObject('PROJECT WEB/location/wrong format in edit/input_Fax_locationfax'), 'sdsdssdsdsds')

WebUI.click(findTestObject('PROJECT WEB/location/wrong format in edit/input__btnSave'))

WebUI.click(findTestObject('PROJECT WEB/location/wrong format in edit/span_Allows numbers and only  -  ( )'))

WebUI.click(findTestObject('PROJECT WEB/location/wrong format in edit/span_Allows numbers and only  -  ( )'))

WebUI.closeBrowser()

