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

WebUI.click(findTestObject('Object Repository/PROJECT WEB/location2/b_Admin'))

WebUI.click(findTestObject('Object Repository/PROJECT WEB/location2/a_Organization'))

WebUI.click(findTestObject('Object Repository/PROJECT WEB/location2/a_Locations'))

WebUI.click(findTestObject('Object Repository/PROJECT WEB/location2/input_Country_btnAdd'))

WebUI.setText(findTestObject('Object Repository/PROJECT WEB/location2/input__locationname'), 'Darulaman City Plaza')

WebUI.selectOptionByValue(findTestObject('Object Repository/PROJECT WEB/location2/select_-- Select --AfghanistanAlbaniaAlgeri_3356b1'), 
    'AT', true)

WebUI.setText(findTestObject('Object Repository/PROJECT WEB/location2/input_StateProvince_locationprovince'), 'Jitra')

WebUI.setText(findTestObject('Object Repository/PROJECT WEB/location2/input_City_locationcity'), 'Bandar Darulaman')

WebUI.setText(findTestObject('Object Repository/PROJECT WEB/location2/textarea_Address_locationaddress'), '399, Jalan Supreme, 06000, Jitra')

WebUI.setText(findTestObject('Object Repository/PROJECT WEB/location2/input_ZipPostal Code_locationzipCode'), '06000')

WebUI.click(findTestObject('Object Repository/PROJECT WEB/location2/ol_Name   Country   -- Select --Afghanistan_502f77'))

WebUI.setText(findTestObject('Object Repository/PROJECT WEB/location2/input_Phone_locationphone'), 'dadadasdasdad')

WebUI.click(findTestObject('Object Repository/PROJECT WEB/location2/input__btnSave'))

WebUI.click(findTestObject('Object Repository/PROJECT WEB/location2/span_Allows numbers and only  -  ( )'))

WebUI.closeBrowser()

