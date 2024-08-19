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
import org.assertj.core.api.Assertions as Assertions

'Fully EKYC'
ekycreq_id = ((Math.random() * 100) as int)

custloan_id = ((Math.random() * 100) as int)

println(ekycreq_id)

response6 = WS.sendRequest(findTestObject('EKYC Flow/Fully EKYC', [('ekyc_loan_id') : custloan_id, ('ekyc_req_id') : ekycreq_id, ('NRIC') : New]))

println(response6.getResponseBodyContent())

Assertions.assertThat(response6.getStatusCode()).isEqualTo(200)

web = WS.getElementPropertyValue(response6, 'webUrl')

println(web)

WebUI.openBrowser(web)

WebUI.delay(60)

//WS.sendRequest(findTestObject('EKYC Flow/Fully EKYC', [('ekyc_loan_id') : 1, ('ekyc_req_id') : 133434545, ('NRIC') : 620324030178]))

