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
println("**************CUSTOMER RISK RATING STARTED**************")
custreq_id = (long) Math.floor(Math.random() * 9_000_000_000L) + 1_000_000_000L
custloan_id = (long) Math.floor(Math.random() * 9_000_000_000L) + 1_000_000_000L
println(custloan_id)
response1 = WS.sendRequest(findTestObject('EKYC Flow/Customer Risk Rating', [('cust_req_id') : custreq_id, ('cust_loan_id') : custloan_id, ('NRIC'): New_Nric ]))
println(response1.getResponseBodyContent())
Assertions.assertThat(response1.getStatusCode()).isEqualTo(200)
println(WS.verifyElementPropertyValue(response1, 'evaluationStatus', 'Approve')) 
println("**************CUSTOMER RISK RATING PASSED**************")


'Industry Risk Rating'
println("**************INDUSTRY RISK RATING STARTED**************")
indreq_id = (long) Math.floor(Math.random() * 9_000_000_000L) + 1_000_000_000L
response2 = WS.sendRequest(findTestObject('EKYC Flow/Industry Risk Rating', [('ind_loan_id') : custloan_id, ('ind_req_id') : indreq_id, ('NRIC'): New_Nric]))
println(response2.getResponseBodyContent())
Assertions.assertThat(response2.getStatusCode()).isEqualTo(200)
println(WS.verifyElementPropertyValue(response2, 'riskAssessmentType', 'PROHIBITED_INDUSTRY'))
println(WS.verifyElementPropertyValue(response2, 'evaluationStatus', 'Reject')) 
println("**************INDUSTRY RISK RATING PASSED**************")


'Company Risk Rating'
println("**************COMPANY RISK RATING STARTED**************")
comreq_id = (long) Math.floor(Math.random() * 9_000_000_000L) + 1_000_000_000L
response3 = WS.sendRequest(findTestObject('EKYC Flow/Company Risk Rating', [('com_loan_id') : custloan_id, ('com_req_id') : comreq_id, ('NRIC'): New_Nric]))
println(response3.getResponseBodyContent())
Assertions.assertThat(response3.getStatusCode()).isEqualTo(200)
println(WS.verifyElementPropertyValue(response3, 'riskAssessmentType', 'COMPANY_RISK'))
println(WS.verifyElementPropertyValue(response3, 'evaluationStatus', 'Reject'))
println("**************COMPANY RISK RATING PASSED**************")

'Member Onboarding Status'
println("**************MEMBER ONBOARDING STARTED**************")
statreq_id = (long) Math.floor(Math.random() * 9_000_000_000L) + 1_000_000_000L
custloan_id = (long) Math.floor(Math.random() * 9_000_000_000L) + 1_000_000_000L
println(custloan_id)
response4 = WS.sendRequest(findTestObject('EKYC Flow/Member Onboarding Status', [('stat_req_id') : statreq_id]))
println(response4.getResponseBodyContent())
Assertions.assertThat(response4.getStatusCode()).isEqualTo(200)
println(WS.verifyElementPropertyValue(response4, 'exists', 'true'))
println("**************MEMBER ONBOARDING PASSED**************")


'Member Existence Check'
println("**************MEMBER EXISTENCE STARTED**************")
exreq_id = (long) Math.floor(Math.random() * 9_000_000_000L) + 1_000_000_000L
response5 = WS.sendRequest(findTestObject('EKYC Flow/Member Existence Check', [('ex_loan_id') : custloan_id, ('ex_req_id') : exreq_id, ('NRIC'): New_Nric]))
println(response5.getResponseBodyContent())
Assertions.assertThat(response5.getStatusCode()).isEqualTo(200)
println(WS.verifyElementPropertyValue(response5, 'exists', 'true'))
println("**************MEMBER EXISTENCE PASSED**************") 


'Fully EKYC'
println('**************FULLY EKYC STARTED**************')
ekycreq_id = (((Math.floor(Math.random() * 9000000000)) as long) + 1000000000)
custloan_id = (((Math.floor(Math.random() * 9000000000)) as long) + 1000000000)
response6 = WS.sendRequest(findTestObject('EKYC Flow/Fully EKYC', [('ekyc_loan_id') : custloan_id, ('ekyc_req_id') : ekycreq_id, ('NRIC'): New_Nric]))
println(response6.getResponseBodyContent())
Assertions.assertThat(response6.getStatusCode()).isEqualTo(200)
web = WS.getElementPropertyValue(response6, 'webUrl')
println(web)
WebUI.openBrowser(web)
WebUI.click(findTestObject('Object Repository/WebUI/Page_Netverify - Start verification(fully)/button_Start'))
WebUI.click(findTestObject('Object Repository/WebUI/Page_Netverify - Choose an upload method(fully)/button_Continue on mobile'))
WebUI.delay(120)
WebUI.closeBrowser()
println('**************FULLY EKYC PASSED**************')


'Server Bio'
println('**************SERVER BIO STARTED**************')
serreq_id = (((Math.floor(Math.random() * 9000000000)) as long) + 1000000000)
response7 = WS.sendRequest(findTestObject('EKYC Flow/Server Bio', [('ser_loan_id') : custloan_id, ('ser_req_id') : serreq_id, ('NRIC'): New_Nric]))
println(response7.getResponseBodyContent())
Assertions.assertThat(response7.getStatusCode()).isEqualTo(200)
web1 = WS.getElementPropertyValue(response7, 'webUrl')
println(web1)
WebUI.openBrowser(web1)
WebUI.click(findTestObject('Object Repository/WebUI/Page_Netverify - Start verification(SB)/button_Start'))
WebUI.delay(30)
WebUI.click(findTestObject('Object Repository/WebUI/Page_Netverify - Face verification(SB)/button_Start'))
WebUI.delay(120)
WebUI.closeBrowser()
println('**************SERVER BIO PASSED**************') 

'Loan Onboarding'
println('**************LOAN ONBOARDING STARTED**************')
loanreq_id = (((Math.floor(Math.random() * 9000000000)) as long) + 1000000000)
//loan_id = (((Math.floor(Math.random() * 9000000000)) as long) + 1000000000)
email = (int)(Math.random()*100)
BRN = (((Math.floor(Math.random() * 9000000000)) as long) + 1000000000)

//println(loan_id)
println(loanreq_id)
println(BRN)

Loanresponse = WS.sendRequest(findTestObject('EKYC Flow/Loan Onboarding', [('Token') : GlobalVariable.Token, ('loan_id') : custloan_id, ('loan_req_id') : loanreq_id, ('email') : email, ('BRN') : BRN, ('NRIC'): New_Nric]))
println(Loanresponse.getResponseBodyContent())
Assertions.assertThat(Loanresponse.getStatusCode()).isEqualTo(200)
println(WS.verifyElementPropertyValue(Loanresponse, 'status', 'PROCESSING'))
println('**************LOAN CREATION PASSED**************')


'Loan Account Creation'
println('**************LOAN CREATION STARTED**************')
crreq_id = (((Math.floor(Math.random() * 9000000000)) as long) + 1000000000)
println(custloan_id)
CRResponse = WS.sendRequest(findTestObject('EKYC Flow/Loan Acc Creation', [('Token') : GlobalVariable.Token, ('crloan_id') : custloan_id, ('crreq_id') : crreq_id, ('BRN') : BRN, ('NRIC'): New_Nric]))
println(CRResponse.getResponseBodyContent())
Assertions.assertThat(CRResponse.getStatusCode()).isEqualTo(200)
println(WS.verifyElementPropertyValue(Loanresponse, 'status', 'Processing'))
println(WS.verifyElementPropertyValue(Loanresponse, 'message', 'Loan is being processed'))
WS.delay(20)
println('**************LOAN CREATION PASSED**************')

'Loan Creation Status Check'
println('**************LOAN CREATION STARTED**************')
StatusResponse =WS.sendRequest(findTestObject('EKYC Flow/Loan Creation Status Check', [('Token') : GlobalVariable.Token, ('loan_id') : custloan_id]))
println(StatusResponse.getResponseBodyContent())
Assertions.assertThat(StatusResponse.getStatusCode()).isEqualTo(200)
println(WS.verifyElementPropertyValue(StatusResponse, 'status', 'SUCCESS'))
println(WS.verifyElementPropertyValue(StatusResponse, 'message', 'Loan Account created successfully.'))
println('**************LOAN CREATION PASSED**************')


