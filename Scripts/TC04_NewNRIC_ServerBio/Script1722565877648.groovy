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

'Server Bio'
serreq_id = (((Math.floor(Math.random() * 9000000000)) as long) + 1000000000)
custloan_id = (long) Math.floor(Math.random() * 9_000_000_000L) + 1_000_000_000L
response7 = WS.sendRequest(findTestObject('EKYC Flow/Server Bio', [('ser_loan_id') : custloan_id, ('ser_req_id') : serreq_id, ('NRIC'): New_Nric]))
println(response7.getResponseBodyContent())
Assertions.assertThat(response7.getStatusCode()).isEqualTo(200)
web1 = WS.getElementPropertyValue(response7, 'webUrl')
println(web1)
WebUI.openBrowser(web1)
WebUI.delay(20)
WebUI.closeBrowser()
println('**************SERVER BIO PASSED**************')