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

WebUI.openBrowser('')

WebUI.navigateToUrl('https://opensource-demo.orangehrmlive.com/')

WebUI.setText(findTestObject('PROJECT WEB/location/worng faz/input_LOGIN Panel_txtUsername'), 'Admin')

WebUI.setEncryptedText(findTestObject('PROJECT WEB/location/worng faz/input_Username_txtPassword'), 'Y7oiTuDv42U=')

WebUI.click(findTestObject('PROJECT WEB/location/worng faz/input_Password_Submit'))

WebUI.setText(findTestObject('PROJECT WEB/location/worng faz/input_LOGIN Panel_txtUsername'), 'Admin')

WebUI.setEncryptedText(findTestObject('PROJECT WEB/location/worng faz/input_Username_txtPassword'), 'hUKwJTbofgPU9eVlw/CnDQ==')

WebUI.click(findTestObject('PROJECT WEB/location/worng faz/input_Password_Submit'))

WebUI.click(findTestObject('PROJECT WEB/location/worng faz/b_Admin'))

WebUI.click(findTestObject('PROJECT WEB/location/worng faz/a_Organization'))

WebUI.click(findTestObject('PROJECT WEB/location/worng faz/a_Locations'))

WebUI.click(findTestObject('PROJECT WEB/location/worng faz/input_Country_btnAdd'))

WebUI.setText(findTestObject('PROJECT WEB/location/worng faz/input__locationname'), 'asas')

WebUI.selectOptionByValue(findTestObject('PROJECT WEB/location/worng faz/select_-- Select --AfghanistanAlbaniaAlgeri_3356b1'), 
    'MY', true)

WebUI.setText(findTestObject('PROJECT WEB/location/worng faz/input_StateProvince_locationprovince'), 'Kedah')

WebUI.setText(findTestObject('PROJECT WEB/location/worng faz/input_City_locationcity'), 'Jitra')

WebUI.setText(findTestObject('PROJECT WEB/location/worng faz/textarea_Address_locationaddress'), '399, Jalan Supreme, 06000, Jitra')

WebUI.setText(findTestObject('PROJECT WEB/location/worng faz/input_ZipPostal Code_locationzipCode'), '06000')

WebUI.setText(findTestObject('PROJECT WEB/location/worng faz/input_Fax_locationfax'), 'asasasas')

WebUI.click(findTestObject('PROJECT WEB/location/worng faz/input__btnSave'))

WebUI.doubleClick(findTestObject('PROJECT WEB/location/worng faz/span_Allows numbers and only  -  ( )'))

WebUI.closeBrowser()

