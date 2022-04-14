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

WebUI.click(findTestObject('Object Repository/PROJECT WEB/Page_OrangeHRM/add cadidate/Page_OrangeHRM/input_Method of Application_btnAdd'))

WebUI.setText(findTestObject('Object Repository/PROJECT WEB/Page_OrangeHRM/add cadidate/Page_OrangeHRM/input__addCandidatefirstName'), 
    fname)

WebUI.setText(findTestObject('Object Repository/PROJECT WEB/Page_OrangeHRM/add cadidate/Page_OrangeHRM/input_Middle Name_addCandidatemiddleName'), 
    mname)

WebUI.setText(findTestObject('Object Repository/PROJECT WEB/Page_OrangeHRM/add cadidate/Page_OrangeHRM/input__addCandidatelastName'), 
    lname)

WebUI.setText(findTestObject('Object Repository/PROJECT WEB/Page_OrangeHRM/add cadidate/Page_OrangeHRM/input__addCandidateemail'), 
    email)

WebUI.setText(findTestObject('Object Repository/PROJECT WEB/Page_OrangeHRM/add cadidate/Page_OrangeHRM/input_Contact No_addCandidatecontactNo'), 
    '0198765432')

WebUI.selectOptionByValue(findTestObject('Object Repository/PROJECT WEB/Page_OrangeHRM/add cadidate/Page_OrangeHRM/select_-- Select --Associate IT ManagerCont_1d5b09'), 
    '2', true)

WebUI.uploadFile(findTestObject('PROJECT WEB/require and add file/input_Resume_addCandidateresume'), 'C:\\Users\\syazw\\Documents\\GitHub\\resume small.pdf')

WebUI.setText(findTestObject('Object Repository/PROJECT WEB/Page_OrangeHRM/add cadidate/Page_OrangeHRM/input_Keywords_addCandidatekeyWords'), 
    'Good')

WebUI.setText(findTestObject('Object Repository/PROJECT WEB/Page_OrangeHRM/add cadidate/Page_OrangeHRM/textarea_Comment_addCandidatecomment'), 
    'I\'m good and smart')

WebUI.click(findTestObject('Object Repository/PROJECT WEB/Page_OrangeHRM/add cadidate/Page_OrangeHRM/input__btnSave'))

WebUI.click(findTestObject('Object Repository/PROJECT WEB/Page_OrangeHRM/add cadidate/Page_OrangeHRM/div_Successfully Saved       Close'))

WebUI.click(findTestObject('Object Repository/PROJECT WEB/Page_OrangeHRM/add cadidate/Page_OrangeHRM/input__btnBack'))

