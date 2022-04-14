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

WebUI.click(findTestObject('PROJECT WEB/location/b_Admin'))

WebUI.click(findTestObject('PROJECT WEB/location/a_Organization'))

WebUI.click(findTestObject('PROJECT WEB/location/a_Locations'))

WebUI.click(findTestObject('PROJECT WEB/location/input_Country_btnAdd'))

WebUI.click(findTestObject('PROJECT WEB/location/input__locationname'))

WebUI.setText(findTestObject('PROJECT WEB/location/input__locationname'), locaname)

WebUI.selectOptionByValue(findTestObject('PROJECT WEB/location/select_-- Select --AfghanistanAlbaniaAlgeri_3356b1_1'), 'MY', 
    true)

WebUI.setText(findTestObject('PROJECT WEB/location/input_StateProvince_locationprovince'), 'Kota Setar')

WebUI.setText(findTestObject('PROJECT WEB/location/input_City_locationcity'), 'Alor Setar')

WebUI.setText(findTestObject('PROJECT WEB/location/textarea_Address_locationaddress'), '12, Jalan Supreme, 06000, Alor Setar, Kedah')

WebUI.setText(findTestObject('PROJECT WEB/location/input_ZipPostal Code_locationzipCode'), '06000')

WebUI.setText(findTestObject('PROJECT WEB/location/input_Phone_locationphone'), '012435455')

WebUI.setText(findTestObject('PROJECT WEB/location/textarea_Notes_locationnotes'), 'This place is very nice as castle')

WebUI.click(findTestObject('PROJECT WEB/location/input__btnSave'))

WebUI.click(findTestObject('PROJECT WEB/location/div_Successfully Saved       Close'))

WebUI.closeBrowser()

