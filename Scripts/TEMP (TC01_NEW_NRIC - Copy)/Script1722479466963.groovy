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


'Customer Risk Rating'
custreq_id = (long) Math.floor(Math.random() * 9_000_000_000L) + 1_000_000_000L

custloan_id = (long) Math.floor(Math.random() * 9_000_000_000L) + 1_000_000_000L
println(custloan_id)
response1 = WS.sendRequest(findTestObject('EKYC Flow/Customer Risk Rating', [('cust_req_id') : custreq_id, ('cust_loan_id') : custloan_id]))

println(response1.getResponseBodyContent())

Assertions.assertThat(response1.getStatusCode()).isEqualTo(200)

println(WS.verifyElementPropertyValue(response1, 'evaluationStatus', 'Approve')) 
println("**************CUSTOMER RISK RATING PASSED**************")


'Industry Risk Rating'
indreq_id = (long) Math.floor(Math.random() * 9_000_000_000L) + 1_000_000_000L
response2 = WS.sendRequest(findTestObject('EKYC Flow/Industry Risk Rating', [('ind_loan_id') : custloan_id, ('ind_req_id') : indreq_id]))

println(response2.getResponseBodyContent())
Assertions.assertThat(response2.getStatusCode()).isEqualTo(200)
println(WS.verifyElementPropertyValue(response2, 'riskAssessmentType', 'PROHIBITED_INDUSTRY'))
println(WS.verifyElementPropertyValue(response2, 'evaluationStatus', 'Reject')) 
println("**************INDUSTRY RISK RATING PASSED**************")


'Company Risk Rating'
comreq_id = (long) Math.floor(Math.random() * 9_000_000_000L) + 1_000_000_000L
response3 = WS.sendRequest(findTestObject('EKYC Flow/Company Risk Rating', [('com_loan_id') : custloan_id, ('com_req_id') : comreq_id]))
println(response3.getResponseBodyContent())
Assertions.assertThat(response3.getStatusCode()).isEqualTo(200)
println(WS.verifyElementPropertyValue(response3, 'riskAssessmentType', 'COMPANY_RISK'))
println(WS.verifyElementPropertyValue(response3, 'evaluationStatus', 'Reject'))
println("**************COMPANY RISK RATING PASSED**************")
/*'Member Onboarding Status'
statreq_id = (long) Math.floor(Math.random() * 9_000_000_000L) + 1_000_000_000L
custloan_id = (long) Math.floor(Math.random() * 9_000_000_000L) + 1_000_000_000L
println(custloan_id)
response4 = WS.sendRequest(findTestObject('EKYC Flow/Member Onboarding Status', [('stat_req_id') : statreq_id]))

println(response4.getResponseBodyContent())
Assertions.assertThat(response4.getStatusCode()).isEqualTo(200)*/


'Member Existence Check'
exreq_id = (long) Math.floor(Math.random() * 9_000_000_000L) + 1_000_000_000L
response5 = WS.sendRequest(findTestObject('EKYC Flow/Member Existence Check', [('ex_loan_id') : custloan_id, ('ex_req_id') : exreq_id]))
println(response5.getResponseBodyContent())
Assertions.assertThat(response5.getStatusCode()).isEqualTo(200)
println(WS.verifyElementPropertyValue(response5, 'exists', 'true'))
println("**************MEMBER EXISTENCE PASSED**************") 


'Fully EKYC'
ekycreq_id = (((Math.floor(Math.random() * 9000000000)) as long) + 1000000000)

custloan_id = (((Math.floor(Math.random() * 9000000000)) as long) + 1000000000)

response6 = WS.sendRequest(findTestObject('EKYC Flow/Fully EKYC', [('ekyc_loan_id') : custloan_id, ('ekyc_req_id') : ekycreq_id]))

println(response6.getResponseBodyContent())

Assertions.assertThat(response6.getStatusCode()).isEqualTo(200)

web = WS.getElementPropertyValue(response6, 'webUrl')

println(web)

WebUI.openBrowser(web)

WebUI.delay(180)

WebUI.closeBrowser()

println('**************FULLY EKYC PASSED**************')



'Server Bio'
serreq_id = (((Math.floor(Math.random() * 9000000000)) as long) + 1000000000)

response7 = WS.sendRequest(findTestObject('EKYC Flow/Server Bio', [('ser_loan_id') : custloan_id, ('ser_req_id') : serreq_id]))
println(response7.getResponseBodyContent())

Assertions.assertThat(response7.getStatusCode()).isEqualTo(200)

web1 = WS.getElementPropertyValue(response7, 'webUrl')

println(web1)

WebUI.openBrowser(web1)

WebUI.delay(90)

WebUI.closeBrowser()
println('**************SERVER BIO PASSED**************')
